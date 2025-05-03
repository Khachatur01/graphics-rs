mod render;

use core::entity::Id;
use core::entity::{Entity, Model};
use rendering::behaviour::Render;

pub struct ContainerEntity {
    entities: Vec<Entity>,
}

impl Model for ContainerEntity {}

impl ContainerEntity {
    pub fn new(id: impl Id + 'static) -> Entity {
        let mut entity = Entity::new(
            id,
            ContainerEntity {
                entities: vec![]
            }
        );

        entity.add_behaviour(Render {
            render: render::render
        });

        entity
    }

    pub fn add_entity<M>(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
