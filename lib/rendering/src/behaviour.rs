use crate::Behaviour;
use crate::Entity;
use crate::Renderer;

pub struct Render<Id> {
    pub render: fn(entity: &Entity<Id>, renderer: &mut dyn Renderer),
}

impl<Id> Behaviour for Render<Id> {}
