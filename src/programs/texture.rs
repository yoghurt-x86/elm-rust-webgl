use wasm_bindgen::JsCast;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_funcs as cf;
use super::super::app_state::AppState;
use nalgebra as na;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Texture {
    program: WebGlProgram,
    rect_vertice_ary_length: usize,
    rect_vertice_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    uv_buffer: WebGlBuffer,
    pub texture: WebGlTexture,
    u_transform: WebGlUniformLocation,
    index_count: i32,
    a_position: i32,
    a_texcoord: i32,
}

#[allow(dead_code)]
impl Texture {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::texture::SHADER,
            super::super::shaders::fragment::texture::SHADER,
        ).unwrap();

        let vertices_rect: [f32; 12] = [
            0., 32., 0.,// x, y
            0., 0., 0.,// x, y
            32., 32., 0.,// x, y
            32., 0., 0.,// x, y
        ];

        let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let uv_coords:  [f32; 8] = [ 
            1., 1.,
            1., 0.,
            0., 1.,
            0., 0.,
        ];

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

        let uv_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let uv_location = uv_coords.as_ptr() as u32 / 4;
        let uv_array = js_sys::Float32Array::new(&uv_memory_buffer).subarray(
            uv_location,
            uv_location + uv_coords.len() as u32,
        );

        let buffer_texture = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_texture));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &uv_array, 
            GL::STATIC_DRAW,
            );


        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        let img : [u8; 3] = [
            255, 120, 25, 
        ];

        let img_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let img_location = img.as_ptr() as u32;
        let img_array = js_sys::Uint8Array::new(&img_memory_buffer).subarray(
            img_location,
            img_location + img.len() as u32,
        );
        
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D, //target 
            0,
            GL::RGB as i32,  //inernalFormat
            1, 
            1,
            0,
            GL::RGB,  
            GL::UNSIGNED_BYTE,
            Some(&img_array),
        ).unwrap();

        Self {
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            a_position: gl.get_attrib_location(&program, "aPosition"),
            a_texcoord: gl.get_attrib_location(&program, "a_texcoord"),
            index_count: indices_array.length() as i32,
            index_buffer:  buffer_indices,
            rect_vertice_ary_length: vertices_rect.len(),
            rect_vertice_buffer: buffer_rect,
            texture: texture,
            uv_buffer: buffer_texture,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        app: &AppState,
    ) {


        

        let rotate_flat = na::Rotation3::from_euler_angles((app.time / 13.) * std::f32::consts::PI / 180., (app.time / 23.) * std::f32::consts::PI / 180., 0.).to_homogeneous();
        let translate_y = na::Matrix4::new_translation(&na::Vector3::new(0.,0.5,0.));
        let model_transform = translate_y * rotate_flat;

        let camera_point = app.camera.position;
        let camera_dir = app.camera.direction;
        let target = camera_point + camera_dir;

        let up = na::Vector3::new(0.,0.,1.);

        let view = na::geometry::Isometry3::look_at_rh(&camera_point, &target, &up);
        let perspective : na::geometry::Perspective3<f32> = na::geometry::Perspective3::new(app.canvas_width / app.canvas_height, app.camera.fov * std::f32::consts::PI / 180. /*(90deg)*/, 1.0, 100000.0);
        let camera_transform = perspective.as_matrix() * view.to_homogeneous();
        let transform =  camera_transform * model_transform ; //* translation * scale;

        gl.use_program(Some(&self.program));

        // positions
        gl.enable_vertex_attrib_array(self.a_position as u32);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
        gl.vertex_attrib_pointer_with_i32(self.a_position as u32, 3, GL::FLOAT, false, 0, 0);

        //index
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

        //texture
        gl.enable_vertex_attrib_array(self.a_texcoord as u32);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.uv_buffer));
        gl.vertex_attrib_pointer_with_i32(self.a_texcoord as u32, 2, GL::FLOAT, false, 0, 0);

        //transform
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, transform.as_slice());
        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}
