use wasm_bindgen::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use nalgebra as na;
use super::Movement;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn get_curr_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: na::Point3<f32>,
    pub direction: na::Vector3<f32>,
    pub euler_angles: na::Vector3<f32>,
    pub fov: f32,
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub control_bottom: f32,
    pub control_top: f32,
    pub control_left: f32,
    pub control_right: f32,
    pub time: f32,
    pub camera : Camera,
}


#[wasm_bindgen]
pub struct OutMsg {
    pub time: f32,
    pub fps: f32,
}

impl Camera {
    fn new() -> Self {
        let default = na::Vector3::new(1., 0., 0.);
        let angles = na::Vector3::new(
            0., 
            21. * std::f32::consts::PI / 180.,
            214. * std::f32::consts::PI / 180.
            );
        let rotate_camera = na::Rotation3::from_euler_angles(angles.x, angles.y, angles.z).to_homogeneous();
        let what : na::Vector3<f32> = rotate_camera.transform_vector(&default);
        Self {
            position: na::Point3::new(128., 128., 64.),
            direction: what,
            euler_angles: angles,
            fov: 90.0,
        }
    }
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0., 
            canvas_width: 0.,
            control_bottom: 0.,
            control_top: 0.,
            control_left: 0.,
            control_right: 0.,
            time: 0.,
            camera: Camera::new(),
        }
    }
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32, keys: &Vec<String>, viewport_active: bool, mouse_movement: &Movement, angle: Option<f32>) -> OutMsg {
    let min_height_width = canvas_height.min(canvas_width);
    let display_size = 0.9 * min_height_width;
    let half_display_size = display_size / 2.;
    let half_canvas_height = canvas_height / 2.;
    let half_canvas_width = canvas_width / 2.;

    let mut data = APP_STATE.lock().unwrap();
    
    let new_camera = 
        if viewport_active {
            let camera = data.camera;
            let default = na::Vector3::new(1., 0., 0.);
            let angles = na::Vector3::new(
                0., 
                camera.euler_angles.y + (mouse_movement.y * 0.001), 
                camera.euler_angles.z + (mouse_movement.x * -0.001)
                );
            let camera_rotation = na::Rotation3::from_euler_angles(angles.x, angles.y, angles.z).to_homogeneous();
            let what : na::Vector3<f32> = camera_rotation.transform_vector(&default);

            let new_dir = what;//camera_rotation.transform_vector(&camera.direction);


            let mut point = camera.position;
            let dir = new_dir;
            let up = na::Vector3::new(0.,0.,1.);

            for k in keys {
                match k.as_str() {
                    "w" => point = point + (dir * 2.),
                    "s" => point = point + (dir * -2.),
                    "d" => point = point + ((dir.cross(&up)).normalize() * 2.),
                    "a" => point = point + ((dir.cross(&up)).normalize() * -2.),
                     _ =>  (),
                }
            }

            let fov_angle = angle.unwrap_or(camera.fov);

            Camera { position: point, direction: dir, euler_angles: angles, fov: fov_angle }
        } else {
            let fov_angle = angle.unwrap_or(data.camera.fov);
            Camera {
                fov: fov_angle,
                ..data.camera
            }
        };

    let fps = 1000. / (time - data.time);

    *data = Arc::new(AppState {
        canvas_height: canvas_height,
        canvas_width: canvas_width,

        control_bottom: half_canvas_height - half_display_size,
        control_top: half_canvas_height + half_display_size,
        control_left: half_canvas_width - half_display_size,
        control_right: half_canvas_width + half_display_size,

        time: time,
        camera: new_camera, 
        ..*data.clone()
    });

    return OutMsg { time: time, fps: fps}
}

