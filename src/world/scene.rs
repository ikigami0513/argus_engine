use crate::{graphics::shader::Shader, world::entity::Entity};

use super::skybox::SkyBox;

pub struct Scene {
    pub entities: Vec<Entity>,
    pub skybox: SkyBox
}

impl Scene {
    pub fn new(skybox_shader: &Shader) -> Scene {
        let skybox = unsafe {
            SkyBox::new(
          &[
                    "resources/textures/skybox/right.jpg",
                    "resources/textures/skybox/left.jpg",
                    "resources/textures/skybox/top.jpg",
                    "resources/textures/skybox/bottom.jpg",
                    "resources/textures/skybox/back.jpg",
                    "resources/textures/skybox/front.jpg"
                ],
                &skybox_shader
            )
        };

        Scene {
            entities: Vec::new(),
            skybox
        }
    }

    pub fn render(&mut self, shader: &Shader) {
        for entity in self.entities.iter_mut() {
            if let Some(model) = &mut entity.model {
                unsafe {
                    model.render(&entity.transform, shader);
                }
            }
        }
    }

    pub fn update(&mut self) {
        for entity in self.entities.iter_mut() {
            entity.update();
        }
    }
}