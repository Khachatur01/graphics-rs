mod behaviour;

use crate::behaviour::AddChild;
use crate::entity::container_entity::behaviour::{add_child, render};
use core::entity::Id;
use core::entity::{Entity, Model};
use plugin_rendering::behaviour::Render;

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

        entity.add_behaviour(Render {
            render: render::render
        });

        entity.add_behaviour(AddChild {
            add_child: add_child::add_child
        });

        entity
    }
}
