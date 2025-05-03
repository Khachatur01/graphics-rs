use crate::Behaviour;
use crate::Entity;
use crate::Renderer;

pub struct Render {
    pub render: fn(entity: &Entity, renderer: &mut dyn Renderer),
}

impl Behaviour for Render {}
