use core::entity::Entity;
use core_derive::Feature;

pub mod model;

#[derive(Feature)]
pub struct AddChild {
    pub add_child: fn(entity: &mut Entity, child: Entity),
}
