use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::app_state;

use super::super::common_funcs as cf;
use super::super::app_state::AppState;
use super::super::smd::Mesh;
use nalgebra as na;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MeshProgram {
    skybox: WebGlProgram,
    program: WebGlProgram,
    u_perspective: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_light_direction: WebGlUniformLocation,
    u_ambient_light: WebGlUniformLocation,
    u_env_light: WebGlUniformLocation,
    u_texture1: WebGlUniformLocation,
    u_texture2: WebGlUniformLocation,
    a_position: i32,
    a_normal: i32,
    a_texcoord: i32,
    buffers: Vec<MeshBuffer>,
    depth_texture_buffer: WebGlTexture,
    skybox_a_position: i32,
    skybox_u_skybox: WebGlUniformLocation,
    skybox_texture: WebGlTexture,
    skybox_buffer: WebGlBuffer,
    skybox_u_view_direction_projection_inverse: WebGlUniformLocation,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MeshBuffer {
    positions_size: usize,
    positions_buffer: WebGlBuffer,
    normals_size: usize,
    normals_buffer: WebGlBuffer,
    uv_buffer: WebGlBuffer,
    uv_size: usize,
    texture_buffer: WebGlTexture,
}


impl MeshProgram {
    pub fn new(
        gl: &WebGlRenderingContext, 
        meshes: &Vec<(String,Mesh)>, 
        textures: &Vec<HtmlImageElement>, 
        skybox: &Vec<HtmlImageElement>,
        depth_texture: &HtmlImageElement, 
        ) -> Self {
        let skybox_program = cf::link_program(
            &gl,
            super::super::shaders::vertex::skybox::SHADER,
            super::super::shaders::fragment::skybox::SHADER,
        ).unwrap();

        let skybox_positions : [f32;12] = [
              -1.0, -1.0, 
               1.0, -1.0,  
              -1.0,  1.0, 
              -1.0,  1.0, 
               1.0, -1.0, 
               1.0,  1.0, 
            ]; 

        let skybox_memory = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let skybox_location = skybox_positions.as_ptr() as u32 / 4;
        let skybox_array = js_sys::Float32Array::new(&skybox_memory).subarray(
            skybox_location,
            skybox_location + skybox_positions.len() as u32,
        );
        let skybox_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&skybox_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &skybox_array, GL::STATIC_DRAW);
        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);
        let skybox_texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_CUBE_MAP, Some(&skybox_texture));
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_CUBE_MAP_POSITIVE_Z, //target 
            0,
            GL::RGBA as i32,  //inernalFormat
            GL::RGBA,  
            GL::UNSIGNED_BYTE,
            &skybox[0],
        ).unwrap();
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_CUBE_MAP_NEGATIVE_Z, //target 
            0,
            GL::RGBA as i32,  //inernalFormat
            GL::RGBA,  
            GL::UNSIGNED_BYTE,
            &skybox[4],
        ).unwrap();
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_CUBE_MAP_POSITIVE_X, //target 
            0,
            GL::RGBA as i32,  //inernalFormat
            GL::RGBA,  
            GL::UNSIGNED_BYTE,
            &skybox[3],
        ).unwrap();
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_CUBE_MAP_NEGATIVE_X, //target 
            0,
            GL::RGBA as i32,  //inernalFormat
            GL::RGBA,  
            GL::UNSIGNED_BYTE,
            &skybox[5],
        ).unwrap();
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_CUBE_MAP_POSITIVE_Y, //target 
            0,
            GL::RGBA as i32,  //inernalFormat
            GL::RGBA,  
            GL::UNSIGNED_BYTE,
            &skybox[1],
        ).unwrap();
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_CUBE_MAP_NEGATIVE_Y, //target 
            0,
            GL::RGBA as i32,  //inernalFormat
            GL::RGBA,  
            GL::UNSIGNED_BYTE,
            &skybox[2],
        ).unwrap();
        gl.generate_mipmap(GL::TEXTURE_CUBE_MAP);
        gl.tex_parameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_MIN_FILTER, GL::LINEAR_MIPMAP_LINEAR as i32);
        
        let skybox_a_position = gl.get_attrib_location(&skybox_program, "a_position");
        let skybox_u_skybox = gl.get_uniform_location(&skybox_program, "u_skybox").unwrap();
        let skybox_u_view_direction_projection_inverse = gl.get_uniform_location(&skybox_program, "u_view_direction_projection_inverse").unwrap();


        let program = cf::link_program(
            &gl,
            super::super::shaders::vertex::mesh::SHADER,
            super::super::shaders::fragment::mesh::SHADER,
        ).unwrap();


        let depth_texture_buffer = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&depth_texture_buffer));
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D, //target 
            0,
            GL::RGBA as i32,  //inernalFormat
            GL::RGBA,  
            GL::UNSIGNED_BYTE,
            &depth_texture,
        ).unwrap();
        gl.generate_mipmap(GL::TEXTURE_2D);


        let mut buffers = Vec::<MeshBuffer>::new();
        for (i ,(_name, mesh)) in meshes.iter().enumerate() {
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
            skybox_buffer: skybox_buffer,
            skybox_a_position: skybox_a_position,
            skybox_texture: skybox_texture,
            skybox_u_skybox: skybox_u_skybox,
            skybox_u_view_direction_projection_inverse: skybox_u_view_direction_projection_inverse,
            a_position: gl.get_attrib_location(&program, "a_position"),
            a_normal: gl.get_attrib_location(&program, "a_normal"),
            a_texcoord: gl.get_attrib_location(&program, "a_texcoord"),
            u_perspective: gl.get_uniform_location(&program, "u_perspective").unwrap(),
            u_view: gl.get_uniform_location(&program, "u_view").unwrap(),
            u_light_direction: gl.get_uniform_location(&program, "u_reverse_light").unwrap(),
            u_ambient_light: gl.get_uniform_location(&program, "u_ambient_light").unwrap(),
            u_env_light: gl.get_uniform_location(&program, "u_env_light").unwrap(),
            u_texture1: gl.get_uniform_location(&program, "u_texture1").unwrap(),
            u_texture2: gl.get_uniform_location(&program, "u_texture2").unwrap(),

            depth_texture_buffer: depth_texture_buffer,

            buffers: buffers,
            program: program,
            skybox: skybox_program,
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

        let mut view_inverse = view.to_homogeneous();
        view_inverse[12] = 0.0;
        view_inverse[13] = 0.0;
        view_inverse[14] = 0.0;

        let view_direction = perspective.as_matrix() * view_inverse;

        let camera_transform = perspective.as_matrix() * view.to_homogeneous();
        let view_direction_inverse = view_direction.try_inverse().unwrap_or(na::base::Matrix4::identity());

        gl.use_program(Some(&self.program));
        gl.depth_func(GL::LESS);

        gl.uniform1i(Some(&self.u_texture1), 0);
        gl.uniform1i(Some(&self.u_texture2), 1);

        let light_dir = na::Vector3::new(-0.4, 0.6, 0.6);
        let light_dir_eye_coord = view_inverse.transform_vector(&light_dir).normalize();
        let light_dir_perspective = perspective.as_matrix().transform_point(&na::Point3::from_coordinates(light_dir_eye_coord)); 

        //cf::log(&format!("{:?}", light_dir_perspective));

        gl.uniform3f(
            Some(&self.u_light_direction),
            light_dir_eye_coord.x, light_dir_eye_coord.y, light_dir_eye_coord.z
        );
        gl.uniform3f(
            Some(&self.u_ambient_light),
            app.ambient_light_color.x,
            app.ambient_light_color.y,
            app.ambient_light_color.z,
        );
        gl.uniform3f(
            Some(&self.u_env_light),
            app.env_light_color.x,
            app.env_light_color.y,
            app.env_light_color.z,
        );

        for buffer in &self.buffers {
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

            gl.active_texture(GL::TEXTURE1);
            gl.bind_texture(GL::TEXTURE_2D, Some(&self.depth_texture_buffer));


            //transform
            gl.uniform_matrix4fv_with_f32_array(Some(&self.u_perspective), false, perspective.to_homogeneous().as_slice());
            gl.uniform_matrix4fv_with_f32_array(Some(&self.u_view), false, view.to_homogeneous().as_slice());

            gl.draw_arrays(GL::TRIANGLES, 0, (buffer.positions_size / 3) as i32);
        }

        gl.use_program(Some(&self.skybox));
        gl.depth_func(GL::LEQUAL);

        // positions
        gl.enable_vertex_attrib_array(self.skybox_a_position as u32);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.skybox_buffer));
        gl.vertex_attrib_pointer_with_i32(self.skybox_a_position as u32, 2, GL::FLOAT, false, 0, 0);

        // texture
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_CUBE_MAP, Some(&self.skybox_texture));
        gl.uniform1i(Some(&self.skybox_u_skybox), 0);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.skybox_u_view_direction_projection_inverse), false, view_direction_inverse.as_slice());

        gl.draw_arrays(GL::TRIANGLES, 0, 6 as i32);
    }
}
