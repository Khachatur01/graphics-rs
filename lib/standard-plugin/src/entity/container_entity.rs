mod render;

use crate::entity::container_entity::render::render;
use core::entity::{Entity, Model};
use rendering::behaviour::Render;

pub struct ContainerEntity<Id> {
    entities: Vec<Entity<Id>>,
}

impl<Id> Model for ContainerEntity<Id> {}

impl<Id: 'static> ContainerEntity<Id> {
    pub fn new(id: Id) -> Entity<Id> {
        let mut entity = Entity::new(
            id,
            ContainerEntity::<Id> {
                entities: vec![]
            }
        );

        entity.add_behaviour(Render {
            render: render::<Id>
        });

        entity
    }

    pub fn add_entity<M>(&mut self, entity: Entity<Id>) {
        self.entities.push(entity);
    }
}
