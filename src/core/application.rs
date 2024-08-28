extern crate glfw;
extern crate gl;

use glfw::{ Context, Key, Action };
use std::sync::mpsc::Receiver;
use std::ffi::CStr;
use cgmath::{ perspective, vec3, Deg, Matrix4, Point3 };
use crate::graphics::camera::{ Camera, CameraMovement };
use crate::graphics::model::Model;
use crate::graphics::shader::Shader;
use crate::world::entity::Entity;
use crate::world::scene::Scene;
use crate::world::transform::Transform;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct Application {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    camera: Camera,
    first_mouse: bool,
    last_x: f32,
    last_y: f32,
    delta_time: f32,
    last_frame: f32,
    shader: Shader,
    scene: Scene
}

impl Application {
    pub fn new() -> Self {
        // glfw initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os="macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "Argus Engine",
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);

        // tell GLFW to capture our mouse
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let shader = unsafe {
            // configure global opengl state
            gl::Enable(gl::DEPTH_TEST);

            // build and compile shaders
            Shader::new(
                "src/graphics/shaders/model.vs",
                "src/graphics/shaders/model.fs"
            )
        };

        let mut scene = Scene::default();
        let model = Model::new("resources/objects/nanosuit/nanosuit.obj");
        scene.entities.push(Entity::new(
            Some(model),
            Transform::new(
                vec3(0.0, -1.75, 0.0),
                vec3(0.0, 0.0, 0.0),
                vec3(0.2, 0.2, 0.2)
            )
        ));

        Application {
            glfw,
            window,
            events,
            camera: Camera {
                position: Point3 { x: 0.0, y: 0.0, z: 3.0 },
                ..Camera::default()
            },
            first_mouse: true,
            last_x: SCR_WIDTH as f32 / 2.0,
            last_y: SCR_HEIGHT as f32 / 2.0,
            delta_time: 0.0,
            last_frame: 0.0,
            shader,
            scene
        }
    }

    pub fn run(&mut self) {
        // render loop
        while !self.window.should_close() {
            self.update_delta_time();
            self.process_event();
            self.process_inputs();
            self.render();
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn update_delta_time(&mut self) {
        let current_frame = self.glfw.get_time() as f32;
        self.delta_time = current_frame - self.last_frame;
        self.last_frame = current_frame;
    }

    fn process_event(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let (xpos, ypos) = (xpos as f32, ypos as f32);
                    if self.first_mouse {
                        self.last_x = xpos;
                        self.last_y = ypos;
                        self.first_mouse = false;
                    }

                    let xoffset = xpos - self.last_x;
                    let yoffset = self.last_y - ypos; // reversed since y-coordinates go from bottom to top

                    self.last_x = xpos;
                    self.last_y = ypos;

                    self.camera.process_mouse_movement(xoffset, yoffset, true);
                }
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    self.camera.process_mouse_scroll(yoffset as f32);
                }
                _ => {}
            }
        }
    }

    fn process_inputs(&mut self) {
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_should_close(true);
        }

        if self.window.get_key(Key::W) == Action::Press {
            self.camera.process_keyboard(CameraMovement::FORWARD, self.delta_time);
        }
        if self.window.get_key(Key::S) == Action::Press {
            self.camera.process_keyboard(CameraMovement::BACKWARD, self.delta_time);
        }
        if self.window.get_key(Key::A) == Action::Press {
            self.camera.process_keyboard(CameraMovement::LEFT, self.delta_time);
        }
        if self.window.get_key(Key::D) == Action::Press {
            self.camera.process_keyboard(CameraMovement::RIGHT, self.delta_time);
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            self.shader.use_program();

            // view / projection transformations
            let projection: Matrix4<f32> = perspective(
                Deg(self.camera.zoom),
                SCR_WIDTH as f32 / SCR_HEIGHT as f32,
                0.1,
                100.0
            );
            let view = self.camera.get_view_matrix();
            self.shader.set_mat4(c_str!("projection"), &projection);
            self.shader.set_mat4(c_str!("view"), &view);

            // render the loaded model
            self.scene.render(&self.shader);
        }
    }
}