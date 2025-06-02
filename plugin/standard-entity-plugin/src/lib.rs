use entity_model_feature::entity::Entity;
use entity_model_feature_derive::Feature;

pub mod model;

#[derive(Feature, Clone)]
pub struct AddChild {
    pub add_child: fn(entity: &mut Entity, child: Entity),
}
