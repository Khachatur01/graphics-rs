use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature_derive::Feature;

pub mod model;

#[derive(Feature, Clone)]
pub struct AddChild<Id: EntityId> {
    pub add_child: fn(entity: &mut Entity<Id>, child: Entity<Id>),
}
