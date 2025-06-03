pub mod renderer;
pub mod style;

use crate::renderer::Renderer;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature_derive::Feature;

#[derive(Feature, Clone)]
pub struct Render<Id: EntityId> {
    pub render: fn(entity: &Entity<Id>, renderer: &mut dyn Renderer),
}
