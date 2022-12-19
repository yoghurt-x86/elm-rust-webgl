use wasm_bindgen::prelude::*;
use web_sys::*;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use wasm_bindgen_futures::JsFuture;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{Response, Request, RequestMode, RequestInit};
use futures::future::try_join_all;
use common_funcs as cf;
use elm_rust::Msg;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod programs;
mod common_funcs;
mod smd;
mod shaders;

#[wasm_bindgen]
pub struct Client {
    canvas: web_sys::HtmlCanvasElement,
    gl: Rc<RefCell<WebGlRenderingContext>>,
    program_color_2d: programs::Color2D,
//    program_texture: programs::Texture,
    program_mesh: programs::MeshProgram,
    state: app_state::AppState,
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

pub async fn load_model() -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "./assets/playerstart.txt";
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Accept", "text/plain")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let txt : String = JsFuture::from(resp.text()?).await?.as_string().unwrap();

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(txt)
}

pub async fn load_texture(url: &str) -> Result<HtmlImageElement, JsValue> {
    let promise = js_sys::Promise::new(&mut |resolve: js_sys::Function, _reject: js_sys::Function| {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        image.borrow_mut().set_src(&url);
        let i = image.clone();
        let cb = Closure::once(move |_event : web_sys::Event | {
            resolve.apply(&JsValue::NULL, &js_sys::Array::of1(&i.borrow())).unwrap();
        });
        image.borrow_mut().add_event_listener_with_callback("load", cb.as_ref().unchecked_ref()).unwrap();
        cb.forget()
    });
    let res = wasm_bindgen_futures::JsFuture::from(promise).await?; 
    Ok(res.dyn_into::<HtmlImageElement>().unwrap())
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
        let player = load_model().await.unwrap();

        let mesh = smd::parse_smd(&player).unwrap();
        let assets = mesh.iter().map(|m| format!("./assets/images/{}.png", m.0)).collect::<Vec<String>>();

        let depth_map = load_texture("./assets/images/depth_layer_small.png").await.unwrap();

        let textures = try_join_all(assets.iter().map(|m| load_texture(&m))).await.unwrap();

        let skybox_links = vec!
                ["./assets/images/sky_hr_aztecup.png",
                 "./assets/images/sky_hr_aztecrt.png",
                 "./assets/images/sky_hr_azteclf.png",
                 "./assets/images/sky_hr_aztecft.png",
                 "./assets/images/sky_hr_aztecdn.png",
                 "./assets/images/sky_hr_aztecbk.png",
                ];

        let skybox_textures = try_join_all(skybox_links.iter().map(|src| load_texture(src))).await.unwrap();

        let program_color_2d = programs::Color2D::new(&rgl.borrow());
        //let program_texture = programs::Texture::new(
        //    &rgl.borrow(), 
        //    tex2,
        //);
        let program_mesh = programs::MeshProgram::new(
            &rgl.borrow(), 
            &mesh,
            &textures,
            &skybox_textures,
            &depth_map,
        );

        Self {
            canvas: canvas,
            program_color_2d: program_color_2d,
            //program_texture: program_texture,
            program_mesh: program_mesh,
            gl: rgl,
            state: app_state::AppState::new(),
        }
    }

    pub fn update(
        &mut self, 
        time: f32, 
        canvas_height: f32, 
        canvas_width: f32,
        held_keys: js_sys::Set,
        mouse_movement: &Movement, 
        viewport_active: bool,
        messages: js_sys::Array,
        ) -> Result<app_state::OutMsg, JsValue> {

        let keyboard_input: Vec<String> = {
            let mut keys = Vec::new();
            for val in held_keys.values() {
                keys.push(val?.as_string().unwrap());
            }
            keys
        };

        self.state.update_canvas_dimensions(canvas_height, canvas_width);

        self.state.update_camera(
            time,
            &keyboard_input, 
            viewport_active, 
            mouse_movement);

        for msg in messages.iter() {
            let m : Msg = serde_wasm_bindgen::from_value(msg).unwrap();
            self.state.handle_msg(m, &self.canvas);
        }

        let fps = 1000. / (time - self.state.time);
        self.state.time = time;
        Ok(app_state::OutMsg { 
            time: time, 
            fps: fps, 
            env_light_color_r: self.state.env_light_color.x, 
            env_light_color_g: self.state.env_light_color.y,
            env_light_color_b: self.state.env_light_color.z,
            ambient_light_color_r: self.state.ambient_light_color.x, 
            ambient_light_color_g: self.state.ambient_light_color.y,
            ambient_light_color_b: self.state.ambient_light_color.z,
        })
    }

    pub fn render(&self) {
        let gl = self.gl.borrow_mut();

        gl.enable(GL::CULL_FACE);
        gl.enable(GL::DEPTH_TEST);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = self.state;

        self.program_color_2d.render(
             &gl,
             &curr_state,
         );

        //self.program_texture.render(
        //     &gl,
        //     &curr_state,
        // );
        self.program_mesh.render(
             &gl,
             &curr_state,
         );
    }

}
