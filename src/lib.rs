use wasm_bindgen::prelude::*;
use web_sys::*;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use wasm_bindgen_futures::JsFuture;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{Response, Request, RequestMode, RequestInit};
use web_sys::{Url};
use futures::executor::block_on;
use common_funcs as cf;
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate lazy_static;

mod app_state;
mod programs;
mod common_funcs;
mod shaders;


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Msg {
    Focus,
    Unfocus,
    ChangeFOV { angle: f32},
}

#[wasm_bindgen]
pub struct Client {
    canvas: web_sys::HtmlCanvasElement,
    gl: Rc<RefCell<WebGlRenderingContext>>,
    program_color_2d: programs::Color2D,
    program_texture: Rc<RefCell<programs::Texture>>,
    count: Rc<RefCell<Vec<u32>>>,
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Movement {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Movement {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
        }
    }

    pub fn from(x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

pub async fn load_assets2() -> Result<HtmlImageElement, JsValue> {
    let url = "/assets/images/andi.jpg";

    let promise = js_sys::Promise::new(&mut |resolve: js_sys::Function, reject: js_sys::Function| {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        image.borrow_mut().set_src(&url);
        let i = image.clone();
        let cb = Closure::once(move |event : web_sys::Event | {
            resolve.apply(&JsValue::NULL, &js_sys::Array::of1(&i.borrow())).unwrap();
        });
        image.borrow_mut().add_event_listener_with_callback("load", cb.as_ref().unchecked_ref()).unwrap();
        cb.forget()
    });

    let res = wasm_bindgen_futures::JsFuture::from(promise).await?; 
    Ok(res.dyn_into::<HtmlImageElement>().unwrap())
}

pub async fn load_assets() -> Result<HtmlImageElement, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "/assets/images/andi.jpg";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    //assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let val = JsFuture::from(resp.blob()?).await?;

    let blob: Blob = val.dyn_into().unwrap();

    let asd = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let image = HtmlImageElement::new().unwrap();
    image.set_src(&asd);

    // Send the JSON response back to JS.
    Ok(image)
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub async fn new(element: Element) -> Self {

        let canvas: web_sys::HtmlCanvasElement = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let gl_: WebGlRenderingContext = canvas.get_context("webgl").unwrap().unwrap().dyn_into().unwrap();
        let rgl : Rc<RefCell<WebGlRenderingContext>> = Rc::new(RefCell::new(gl_));


        {
            let gl = rgl.borrow_mut();
            gl.enable(GL::BLEND);
            gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
            gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
            gl.clear_depth(1.);
        }
        let tex2 = load_assets2().await.unwrap();
        cf::log(&format!("height is: {:?}", tex2.height()));

        let program_color_2d = programs::Color2D::new(&rgl.borrow());
        let program_texture = Rc::new(RefCell::new(programs::Texture::new(&rgl.borrow())));



        let texture = Rc::new(RefCell::new(load_assets().await.unwrap()));

        let count2 : Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));

        {
            let c = count2.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                let mut refe = c.borrow_mut();
                refe.push(1);
                cf::log("count");
            });
            canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }

        {
            let t = texture.clone();
            let poo = rgl.clone();
            let program = program_texture.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                let gl = poo.borrow_mut();
                gl.bind_texture(GL::TEXTURE_2D, Some(&program.borrow().texture));
                gl.tex_image_2d_with_u32_and_u32_and_image(
                    GL::TEXTURE_2D, //target 
                    0,
                    GL::RGBA as i32,  //inernalFormat
                    GL::RGBA,  
                    GL::UNSIGNED_BYTE,
                    &t.borrow(),
                ).unwrap();
                gl.generate_mipmap(GL::TEXTURE_2D);
                cf::log("texture");
                cf::log(&format!("texture: {:?}", gl));
                cf::log(&format!("texture: {:?}", t.borrow().height()));
            });
            texture.borrow_mut().add_event_listener_with_callback("load", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }

        Self {
            canvas: canvas,
            program_color_2d: program_color_2d,
            program_texture: program_texture,
            gl: rgl,
            count: count2
        }
    }

    pub fn update(
        &mut self, 
        time: f32, 
        height: f32, 
        width: f32,
        held_keys: js_sys::Set,
        mouse_movement: &Movement, 
        viewport_active: bool,
        messages: js_sys::Array,
        ) -> Result<app_state::OutMsg, JsValue> {

        cf::log(&format!("{:?}", self.count.borrow()));

        let s : Vec<String> = {
            let mut keys = Vec::new();
            for val in held_keys.values() {
                keys.push(val?.as_string().unwrap());
            }
            keys
        };

        let mut new_angle = None;
        for msg in messages.iter() {
            let m : Msg = serde_wasm_bindgen::from_value(msg).unwrap();
            match m {
                Msg::Focus => self.canvas.request_pointer_lock(),
                Msg::Unfocus => window().unwrap().document().unwrap().exit_pointer_lock(),
                Msg::ChangeFOV { angle } => new_angle = Some(angle),
            }
        }

        let out = app_state::update_dynamic_data(time, height, width, &s, viewport_active, mouse_movement, new_angle);
        
        Ok(out)
    }

    pub fn render(&self) {

        let gl = self.gl.borrow_mut();

        gl.enable(GL::CULL_FACE);
        gl.enable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        self.program_color_2d.render(
             &gl,
             &curr_state,
         );

        self.program_texture.borrow_mut().render(
             &gl,
             &curr_state,
         );
    }

}
