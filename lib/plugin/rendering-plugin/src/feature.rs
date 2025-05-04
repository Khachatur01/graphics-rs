use crate::Entity;
use crate::Feature;
use crate::Renderer;

pub struct Render {
    pub render: fn(entity: &Entity, renderer: &mut dyn Renderer),
}

impl Feature for Render {}
