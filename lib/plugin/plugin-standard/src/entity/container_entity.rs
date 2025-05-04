mod feature;

use crate::entity::container_entity::feature::add_features;
use core::entity::Id;
use core::entity::{Entity, Model};

pub struct ContainerEntity {
    pub children: Vec<Entity>,
}

impl Model for ContainerEntity {}

impl ContainerEntity {
    pub fn new(id: impl Id + 'static) -> Entity {
        let mut entity = Entity::new(
            id,
            ContainerEntity {
                children: vec![]
            }
        );

        add_features(&mut entity);

        entity
    }
}
