pub mod renderer;
pub mod style;

use entity_model_feature::entity::Entity;
use entity_model_feature_derive::Feature;
use crate::renderer::Renderer;

#[derive(Feature, Clone)]
pub struct Render {
    pub render: fn(entity: &Entity, renderer: &mut dyn Renderer),
}
