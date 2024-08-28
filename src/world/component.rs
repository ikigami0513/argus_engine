use crate::world::entity::Entity;

pub struct Component {
    pub entity: Entity,
}

impl Component {
    pub fn new(entity: Entity) -> Self {
        Component {
            entity
        }
    }

    pub fn start(&mut self) {}
    pub fn update(&mut self) {}
}
