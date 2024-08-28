use crate::{graphics::shader::Shader, world::entity::Entity};

pub struct Scene {
    pub entities: Vec<Entity>
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            entities: Vec::new()
        }
    }
}

impl Scene {
    pub fn render(&mut self, shader: &Shader) {
        for entity in self.entities.iter_mut() {
            match &mut entity.model {
                Some(model) => unsafe {model.render(entity, shader)},
                None => {}
            }
        }
    }

    pub fn update(&mut self) {
        for entity in self.entities.iter_mut() {
            entity.update();
        }
    }
}