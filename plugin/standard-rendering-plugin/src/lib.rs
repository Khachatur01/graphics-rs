pub mod renderer;
pub mod style;

use crate::renderer::Renderer;
use core::entity::Entity;
use core_derive::Feature;

#[derive(Feature)]
pub struct Render {
    pub render: fn(entity: &Entity, renderer: &mut dyn Renderer),
}
