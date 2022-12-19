use elm_rs::{Elm, ElmEncode, ElmDecode};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Elm, ElmEncode, ElmDecode)]
#[serde(tag = "type")]
pub enum Msg {
    Focus,
    Unfocus,
    ChangeFOV { angle: f32},
    ChangeEnvLight { r : f32, g: f32, b: f32 },
    ChangeAmbientLight { r : f32, g: f32, b: f32 },
}

