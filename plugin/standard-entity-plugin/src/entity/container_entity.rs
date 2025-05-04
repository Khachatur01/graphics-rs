mod feature;

use crate::entity::container_entity::feature::add_features;
use core::entity::Id;
use core::entity::{Entity};
use core_derive::Model;

#[derive(Model)]
pub struct ContainerEntity {
    pub children: Vec<Entity>,
}

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
