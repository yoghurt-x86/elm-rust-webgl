use wasm_bindgen::JsCast;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_funcs as cf;
use super::super::app_state::AppState;
use super::super::smd::Mesh;
use nalgebra as na;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Skybox {
    program: WebGlProgram,
    u_transform: WebGlUniformLocation,
    u_light_direction: WebGlUniformLocation,
    a_position: i32,
    a_normal: i32,
    a_texcoord: i32,
    buffers: Vec<MeshBuffer>,
}

#[derive(Debug)]
pub struct MeshBuffer {
    positions_size: usize,
    positions_buffer: WebGlBuffer,
    normals_size: usize,
    normals_buffer: WebGlBuffer,
    uv_buffer: WebGlBuffer,
    uv_size: usize,
    texture_buffer: WebGlTexture,
}


#[allow(dead_code)]
impl MeshProgram {
    pub fn new(gl: &WebGlRenderingContext, meshes: &Vec<(String,Mesh)>, textures: &Vec<HtmlImageElement>) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::mesh::SHADER,
            super::super::shaders::fragment::mesh::SHADER,
        ).unwrap();

        let mut buffers = Vec::<MeshBuffer>::new();
        for (i ,(name, mesh)) in meshes.iter().enumerate() {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let vertices_location = mesh.positions.as_ptr() as u32 / 4;
            let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
                vertices_location,
                vertices_location + mesh.positions.len() as u32,
            );
            let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);


            let uv_memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let uv_location = mesh.uv.as_ptr() as u32 / 4;
            let uv_array = js_sys::Float32Array::new(&uv_memory_buffer).subarray(
                uv_location,
                uv_location + mesh.uv.len() as u32,
            );

            let uv_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&uv_buffer));
            gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &uv_array, 
                GL::STATIC_DRAW,
                );


            let normals_memory = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let normals_location = mesh.normals.as_ptr() as u32 / 4;
            let normals_array = js_sys::Float32Array::new(&normals_memory).subarray(
                normals_location,
                normals_location + mesh.normals.len() as u32,
            );
            let normals_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&normals_buffer));
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &normals_array, GL::STATIC_DRAW);


            let texture_buffer = gl.create_texture().unwrap();
            gl.bind_texture(GL::TEXTURE_2D, Some(&texture_buffer));
            gl.tex_image_2d_with_u32_and_u32_and_image(
                GL::TEXTURE_2D, //target 
                0,
                GL::RGBA as i32,  //inernalFormat
                GL::RGBA,  
                GL::UNSIGNED_BYTE,
                &textures[i],
            ).unwrap();
            gl.generate_mipmap(GL::TEXTURE_2D);


            buffers.push(
                MeshBuffer{
                    positions_buffer: buffer_rect,
                    positions_size: mesh.positions.len(),
                    normals_buffer: normals_buffer,
                    normals_size: mesh.normals.len(),
                    uv_buffer: uv_buffer,
                    uv_size: mesh.uv.len(),
                    texture_buffer: texture_buffer,
                }
            );
        }

        Self {
            a_position: gl.get_attrib_location(&program, "a_position"),
            a_normal: gl.get_attrib_location(&program, "a_normal"),
            a_texcoord: gl.get_attrib_location(&program, "a_texcoord"),
            u_transform: gl.get_uniform_location(&program, "u_transform").unwrap(),
            u_light_direction: gl.get_uniform_location(&program, "u_reverse_light").unwrap(),
            buffers: buffers,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        app: &AppState,
    ) {
        let camera_point = app.camera.position;
        let camera_dir = app.camera.direction;
        let target = camera_point + camera_dir;

        let up = na::Vector3::new(0.,0.,1.);

        let view = na::geometry::Isometry3::look_at_rh(&camera_point, &target, &up);
        let perspective : na::geometry::Perspective3<f32> = na::geometry::Perspective3::new(app.canvas_width / app.canvas_height, app.camera.fov * std::f32::consts::PI / 180. /*(90deg)*/, 1.0, 100000.0);
        let camera_transform = perspective.as_matrix() * view.to_homogeneous();
        let transform =  camera_transform; //* translation * scale;

        gl.use_program(Some(&self.program));

        for buffer in &self.buffers {

            gl.uniform3f(
                Some(&self.u_light_direction),
                0.2, 0.1, 1.0
            );
            // positions
            gl.enable_vertex_attrib_array(self.a_position as u32);
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer.positions_buffer));
            gl.vertex_attrib_pointer_with_i32(self.a_position as u32, 3, GL::FLOAT, false, 0, 0);

            // normals
            gl.enable_vertex_attrib_array(self.a_normal as u32);
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer.normals_buffer));
            gl.vertex_attrib_pointer_with_i32(self.a_normal as u32, 3, GL::FLOAT, false, 0, 0);

            //uvs
            gl.enable_vertex_attrib_array(self.a_texcoord as u32);
            gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer.uv_buffer));
            gl.vertex_attrib_pointer_with_i32(self.a_texcoord as u32, 2, GL::FLOAT, false, 0, 0);

            // texture
            gl.active_texture(GL::TEXTURE0);
            gl.bind_texture(GL::TEXTURE_2D, Some(&buffer.texture_buffer));

            //transform
            gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, transform.as_slice());

            gl.draw_arrays(GL::TRIANGLES, 0, (buffer.positions_size / 3) as i32);
        }
    }
}
