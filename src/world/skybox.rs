use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ffi::CStr;

use cgmath::Matrix4;

use image;
use image::GenericImage;

use crate::graphics::camera::Camera;
use crate::graphics::shader::Shader;

pub struct SkyBox {
    vao: u32,
    vbo: u32,
    texture: u32
}

impl SkyBox {
    pub unsafe fn new(faces: &[&str], shader: &Shader) -> SkyBox {
        // Setup skybox VAO and VBO
        let skybox_vertices: [f32; 108] = [
            // positions
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0
        ];

        let (mut vao, mut vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (skybox_vertices.len() * mem::size_of::<f32>()) as isize,
                       &skybox_vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as i32, ptr::null());

        // Load cubemap texture
        let texture = SkyBox::load_cubemap(faces);

        // Set texture unit in the shader
        shader.use_program();
        shader.set_int(c_str!("skybox"), 0);

        SkyBox { vao, vbo, texture }
    }

    unsafe fn load_cubemap(faces: &[&str]) -> u32 {
        let mut texture_id = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

        for (i, face) in faces.iter().enumerate() {
            let img = image::open(&Path::new(face)).expect("Cubemap texture failed to load");
            let data = img.raw_pixels();
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                0, gl::RGB as i32, img.width() as i32, img.height() as i32,
                0, gl::RGB, gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void);
        }

        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

        texture_id
    }

    pub unsafe fn draw(&self, projection: Matrix4<f32>, camera: &Camera, shader: &Shader) {
        // Draw skybox
        gl::DepthFunc(gl::LEQUAL);
        shader.use_program();

        let mut view = camera.get_view_matrix();
        view.w[0] = 0.0;
        view.w[1] = 0.0;
        view.w[2] = 0.0;

        shader.set_mat4(c_str!("view"), &view);
        shader.set_mat4(c_str!("projection"), &projection);

        gl::BindVertexArray(self.vao);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
        gl::BindVertexArray(0);
        gl::DepthFunc(gl::LESS);
    }

    pub unsafe fn cleanup(&self) {
        gl::DeleteVertexArrays(1, &self.vao);
        gl::DeleteBuffers(1, &self.vbo);
    }
}