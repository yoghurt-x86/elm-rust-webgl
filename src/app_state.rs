use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement};
use std::sync::Arc;
use std::sync::Mutex;
use core::cmp::max;
use nalgebra as na;
use elm_rust::{Msg, Color};
use crate::Client;

use super::Movement;


#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: na::Point3<f32>,
    pub direction: na::Vector3<f32>,
    pub euler_angles: na::Vector3<f32>,
    pub fov: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub control_bottom: f32,
    pub control_top: f32,
    pub control_left: f32,
    pub control_right: f32,
    pub time: f32,
    pub camera : Camera,
    pub env_light_color: na::Vector3<f32>,
    pub ambient_light_color: na::Vector3<f32>,
    pub skybox: elm_rust::Skybox,
}

#[wasm_bindgen]
pub struct OutMsg {
    pub time: f32,
    pub fps: f32,
}

impl Camera {
    fn new() -> Self {
        Camera::from(90.0)
    }

    fn from(fov: f32) -> Self {
        let default = na::Vector3::new(1., 0., 0.);
        let angles = na::Vector3::new(
            0., 
            7. * std::f32::consts::PI / 180.,
            43. * std::f32::consts::PI / 180.
            );
        let rotate_camera = na::Rotation3::from_euler_angles(angles.x, angles.y, angles.z).to_homogeneous();
        let what : na::Vector3<f32> = rotate_camera.transform_vector(&default);
        Self {
            position: na::Point3::new(-16., -18., 62.),
            direction: what,
            euler_angles: angles,
            fov: fov,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            canvas_height: 0., 
            canvas_width: 0.,
            control_bottom: 0.,
            control_top: 0.,
            control_left: 0.,
            control_right: 0.,
            time: 0.,
            camera: Camera::new(),
            ambient_light_color: na::Vector3::new(0.3, 0.3, 0.4),
            env_light_color: na::Vector3::new(1.0, 1.0, 0.8),
            skybox: elm_rust::Skybox::Bitmap,
        }
    }

    pub fn from(global: elm_rust::Global) -> Self {
        Self {
            canvas_height: 0., 
            canvas_width: 0.,
            control_bottom: 0.,
            control_top: 0.,
            control_left: 0.,
            control_right: 0.,
            time: 0.,
            camera: Camera::from(global.fov),
            ambient_light_color: na::Vector3::new(
                global.ambient_light_color.r, 
                global.ambient_light_color.g, 
                global.ambient_light_color.b,
            ),
            env_light_color: na::Vector3::new(
                global.env_light_color.r, 
                global.env_light_color.g, 
                global.env_light_color.b,
            ),
            skybox: elm_rust::Skybox::Gradient,
        }
    }


    pub fn update_canvas_dimensions(&mut self, 
        canvas_height: f32, 
        canvas_width: f32,) {

        let min_height_width = canvas_height.min(canvas_width);
        let display_size = 0.9 * min_height_width;
        let half_display_size = display_size / 2.;
        let half_canvas_height = canvas_height / 2.;
        let half_canvas_width = canvas_width / 2.;
        
        self.canvas_height = canvas_height;
        self.canvas_width = canvas_width;
        self.control_bottom = half_canvas_height - half_display_size;
        self.control_top = half_canvas_height + half_display_size;
        self.control_left = half_canvas_width - half_display_size;
        self.control_right = half_canvas_width + half_display_size;
    }

    pub fn update_camera(&mut self, 
        time: f32,
        keys: &Vec<String>, 
        viewport_active: bool, 
        mouse_movement: &Movement) {
        if viewport_active {
            let default = na::Vector3::new(1., 0., 0.);
            let angles = na::Vector3::new(
                0., 
                self.camera.euler_angles.y + (mouse_movement.y * 0.001), 
                self.camera.euler_angles.z + (mouse_movement.x * -0.001)
                );
            let camera_rotation = na::Rotation3::from_euler_angles(angles.x, angles.y, angles.z).to_homogeneous();
            let new_dir : na::Vector3<f32> = camera_rotation.transform_vector(&default);

            let mut point = self.camera.position;
            let dir = new_dir;
            let up = na::Vector3::new(0.,0.,1.);

            let delta_time = time - self.time;
            for k in keys {
                match k.as_str() {
                    "w" => point = point + (dir * 0.1 * delta_time),
                    "s" => point = point + (dir * -0.1 * delta_time),
                    "d" => point = point + ((dir.cross(&up)).normalize() * 0.1 * delta_time),
                    "a" => point = point + ((dir.cross(&up)).normalize() * -0.1 * delta_time),
                     _ =>  (),
                }
            }

            self.camera = 
                Camera { 
                    position: point, 
                    direction: dir, 
                    euler_angles: angles, 
                    fov: self.camera.fov,
                };
        } else {
            let camera_rotation = na::Rotation3::from_euler_angles(0.0,0.0,0.001).to_homogeneous();

            let dir = camera_rotation.transform_vector(&self.camera.direction);
            let pos = camera_rotation.transform_point(&self.camera.position);
            self.camera.position = pos;
            self.camera.direction = dir;
            self.camera.euler_angles.z += 0.001;
        }
    }
}

