pub mod renderable;
pub mod style;
pub mod renderer;

use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature_derive::Feature;
use renderer::renderer::Renderer;

#[derive(Feature, Clone)]
pub struct Render<Id: EntityId> {
    pub render: fn(entity: &Entity<Id>, renderer: &mut dyn Renderer),
}
