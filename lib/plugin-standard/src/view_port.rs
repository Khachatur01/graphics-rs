use crate::behaviour::AddChild;
use crate::entity::container_entity::ContainerEntity;
use core::entity::Entity;
use core::entity::Id;
use plugin_rendering::behaviour::Render;
use plugin_rendering::{Renderable, Renderer};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ViewPort {
    container: Arc<RwLock<Entity>>,
}

impl ViewPort {
    pub fn new(id: impl Id + 'static) -> Self {
        Self {
            container: Arc::new(RwLock::new(ContainerEntity::new(id)))
        }
    }

    pub fn add_child(&self, child: Entity) {
        let mut container = self.container.write().expect("Poisoned lock");

        if let Some(add_child) = container.query::<AddChild>() {
            (add_child.add_child)(container.deref_mut(), child);
        }
    }
}

impl Renderable for ViewPort {
    fn render(&self, renderer: &mut dyn Renderer) {
        let container = self.container.try_read().expect("Poisoned lock");

        if let Some(render) = container.query::<Render>() {
            (render.render)(container.deref(), renderer);
        };
    }
}
