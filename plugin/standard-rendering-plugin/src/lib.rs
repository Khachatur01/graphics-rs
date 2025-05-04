pub mod style;
pub mod renderer;

use core::entity::Entity;
use core_derive::Feature;
use crate::renderer::Renderer;

#[derive(Feature)]
pub struct Render {
    pub render: fn(entity: &Entity, renderer: &mut dyn Renderer),
}
