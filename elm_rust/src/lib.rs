use wasm_bindgen::prelude::*;
use elm_rs::{Elm, ElmEncode, ElmDecode};
use serde::{Serialize, Deserialize};
use nalgebra as na;


#[derive(Copy, Clone, Debug, Serialize, Deserialize, Elm, ElmEncode, ElmDecode)]
pub enum Skybox {
    Gradient,
    Bitmap,
}

#[derive(Debug, Serialize, Deserialize, Elm, ElmEncode, ElmDecode)]
pub enum Msg {
    Focus,
    Unfocus,
    ChangeFOV { angle: f32},
    ChangeEnvLight { color: Color},
    ChangeAmbientLight { color: Color },
    SetSkybox { sky: Skybox },
    SetGradient { color1: Color, color2: Color },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Elm, ElmEncode, ElmDecode)]
#[wasm_bindgen]
pub struct Color {
    pub r : f32, 
    pub g: f32, 
    pub b: f32
}

impl Color {
    pub fn new() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn from(vector: na::Vector3<f32>) -> Self {
        Self {
            r: vector.x,
            g: vector.y,
            b: vector.z,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Elm, ElmEncode, ElmDecode)]
#[wasm_bindgen]
pub struct Global {
    pub fov: f32,
    pub env_light_color: Color,
    pub ambient_light_color: Color,
    pub gradient1: Color,
    pub gradient2: Color,
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize, Elm, ElmEncode, ElmDecode)]
pub enum Event {
    Ready
}
