use algebra::linear::transformation::Transformation;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature_derive::Feature;

#[derive(Feature, Clone)]
pub struct Transform<Id: EntityId> {
    pub transform: fn(entity: Entity<Id>, transformation: &Transformation),
}
