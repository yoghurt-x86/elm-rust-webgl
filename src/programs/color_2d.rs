use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_funcs as cf;
use super::super::app_state::AppState;
use nalgebra as na;

#[allow(dead_code)]
pub struct Color2D {
    program: WebGlProgram,
    rect_vertice_ary_length: usize,
    rect_vertice_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
    index_count: i32,
}

#[allow(dead_code)]
impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::color_2d::SHADER,
            super::super::shaders::fragment::color_2d::SHADER,
        ).unwrap();

        let vertices_rect: [f32; 12] = [
            32., 64., 0.,// x, y
            32., 32., 0.,// x, y
            64., 64., 0.,// x, y
            64., 32., 0.,// x, y
        ];

        let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = vertices_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + vertices_rect.len() as u32,
        );
        let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let indices_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let indices_location = indices_rect.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
            indices_location,
            indices_location + indices_rect.len() as u32,
            );

        let buffer_indices = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL::STATIC_DRAW,
            );

        Self {
            u_color: gl.get_uniform_location(&program, "u_color").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "u_opacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "u_transform").unwrap(),
            index_count: indices_array.length() as i32,
            index_buffer:  buffer_indices,
            rect_vertice_ary_length: vertices_rect.len(),
            rect_vertice_buffer: buffer_rect,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        app: &AppState,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(
            Some(&self.u_color),
            0., //r
            0.5,//g
            0.5,//b
            1.0,//a
        );

        gl.uniform1f(Some(&self.u_opacity), 1.);

        let rotate_flat = na::Rotation3::from_euler_angles(0., 0., 32. * std::f32::consts::PI / 180.).to_homogeneous();
        let translate_y = na::Matrix4::new_translation(&na::Vector3::new(0.,0.5,0.));
        let model_transform = translate_y * rotate_flat;

        let camera_point = app.camera.position;
        let camera_dir = app.camera.direction;
        let target = camera_point + camera_dir;

        let up = na::Vector3::new(0.,0.,1.);

        let view = na::geometry::Isometry3::look_at_rh(&camera_point, &target, &up);
        let perspective : na::geometry::Perspective3<f32> = na::geometry::Perspective3::new(app.canvas_width / app.canvas_height, app.camera.fov * std::f32::consts::PI / 180. /*(90deg)*/, 1.0, 100000.0);
        let camera_transform = perspective.as_matrix() * view.to_homogeneous();
        

        let transform =  camera_transform * model_transform ; 


        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, transform.as_slice());

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}
