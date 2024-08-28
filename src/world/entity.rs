use crate::{graphics::model::Model, world::component::Component};
use crate::world::transform::Transform;

pub struct Entity {
    components: Vec<Component>,
    pub transform: Transform,
    pub model: Option<Model>
}

impl Entity {
    pub fn new(model: Option<Model>, transform: Transform) -> Self {
        Entity {
            components: Vec::new(),
            transform,
            model
        }
    }

    pub fn update(&mut self) {
        for component in self.components.iter_mut() {
            component.update();
        }
    }
}
