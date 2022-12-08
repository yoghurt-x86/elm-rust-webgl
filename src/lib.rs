use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod programs;
mod common_funcs;
mod shaders;


#[wasm_bindgen]
pub struct Client {
    canvas: web_sys::HtmlCanvasElement,
    gl: WebGlRenderingContext,
    program_color_2d: programs::Color2D,
    //_program_color_2d_gradient: programs::Color2DGradient,
    //program_graph_3d: programs::Graph3D,
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

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(element: Element) -> Self {

        


        let canvas: web_sys::HtmlCanvasElement = element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let gl: WebGlRenderingContext = canvas.get_context("webgl").unwrap().unwrap().dyn_into().unwrap();

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
        gl.clear_depth(1.);

        Self {
            canvas: canvas,
            program_color_2d: programs::Color2D::new(&gl),
            //_program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            //program_graph_3d: programs::Graph3D::new(&gl),
            gl: gl,
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
        let s : Vec<String> = {
            let mut keys = Vec::new();
            for val in held_keys.values() {
                keys.push(val?.as_string().unwrap());
            }
            keys
        };

        for msg in messages.iter() {
           match msg.as_string().unwrap().as_str() {
               "focus" => self.canvas.request_pointer_lock(),
               "unfocus" => window().unwrap().document().unwrap().exit_pointer_lock(),
               x => panic!("message type not handled: {}", x),

           }
            
        }

        let out = app_state::update_dynamic_data(time, height, width, &s, viewport_active, mouse_movement);


        
        Ok(out)
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        self.program_color_2d.render(
             &self.gl,
             &curr_state,
         );
    }

}
