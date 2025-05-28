use core::entity::Entity;
use standard_rendering_plugin::renderer::{Renderable, Renderer};
use standard_rendering_plugin::Render;
use standard_tool_plugin::tool::{Interaction, Tool};
use standard_tool_plugin::traits::AddEntity;

pub struct ViewPort {
    entities: Vec<Entity>,
    active_tool: Option<Box<dyn Tool>>,
}

impl ViewPort {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            active_tool: None,
        }
    }

    pub fn interaction_event(&mut self, interaction: Interaction) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interaction_event(interaction);
    }

    pub fn activate_tool(&mut self, tool: impl Tool + 'static) {
        self.active_tool = Some(Box::new(tool));
    }
}

impl AddEntity for ViewPort {
    fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

impl Renderable for ViewPort {
    fn render(&self, renderer: &mut dyn Renderer) {
        for entity in &self.entities {
            if let Some(render) = entity.query::<Render>() {
                (render.render)(entity, renderer);
            }
        }

        if let Some(active_tool) = &self.active_tool {
            active_tool.render(renderer);
        }
    }
}
