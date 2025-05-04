use core::entity::Entity;
use core::entity::Id;
use standard_rendering_plugin::Render;
use standard_tool_plugin::tool::Tool;
use std::sync::{Arc, RwLock};
use standard_rendering_plugin::renderer::{Renderable, Renderer};
use standard_tool_plugin::traits::{AddEntity, GetEntities};
use crate::core::container::Container;

struct ViewPortInternal {
    container: Container,
    active_tool: Option<Box<dyn Tool>>,
}

#[derive(Clone)]
pub struct ViewPort {
    internal: Arc<RwLock<ViewPortInternal>>,
}

impl ViewPort {
    pub fn new(id: impl Id + 'static) -> Self {
        Self {
            internal: Arc::new(RwLock::new(
                ViewPortInternal {
                    container: Container::new(id),
                    active_tool: None,
                }
            ))
        }
    }

    pub fn activate_tool(&mut self, tool: impl Tool + 'static) {
        let mut internal = self.internal.write().expect("Poisoned lock");

        internal.active_tool = Some(Box::new(tool));
    }
}

impl AddEntity for ViewPort {
    fn add_entity(&mut self, entity: Entity) {
        let mut internal = self.internal.write().expect("Poisoned lock");

        internal.container.add_entity(entity);
    }
}

// impl GetEntities for ViewPort {
//     fn get_entities(&self) -> &Vec<Entity> {
//         let mut internal = self.internal.write().expect("Poisoned lock");
//
//         &internal.container.entities()
//     }
// }

impl Renderable for ViewPort {
    fn render(&self, renderer: &mut dyn Renderer) {
        let mut internal = self.internal.write().expect("Poisoned lock");

        for entity in internal.container.entities() {
            if let Some(render) = entity.query::<Render>() {
                (render.render)(entity, renderer);
            }
        }

        if let Some(active_tool) = &internal.active_tool {
            active_tool.render(renderer);
        }
    }
}
