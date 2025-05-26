use crate::core::container::Container;
use core::entity::Entity;
use core::entity::Identifier;
use geometry::figure::point::Point;
use standard_rendering_plugin::renderer::{Renderable, Renderer};
use standard_rendering_plugin::Render;
use standard_tool_plugin::tool::{Interaction, PointingDevice, Tool};
use standard_tool_plugin::traits::AddEntity;

pub struct ViewPort<Id> {
    container: Container<Id>,
    active_tool: Option<Box<dyn Tool>>,
}

impl<Id: Identifier> ViewPort<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            container: Container::new(id),
            active_tool: None,
        }
    }

    pub fn id(&self) -> &Id {
        self.container.id()
    }

    pub fn interaction_event(&mut self, interaction: Interaction) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interaction_event(interaction);
    }

    pub fn mouse_down(&mut self, point: Point) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interaction_event(Interaction::PointerDown(point, PointingDevice::Mouse));
    }

    pub fn mouse_move(&mut self, point: Point) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interaction_event(Interaction::PointerMove(point, PointingDevice::Mouse));
    }

    pub fn mouse_up(&mut self, point: Point) {
        let Some(active_tool) = &mut self.active_tool else {
            return;
        };

        active_tool.interaction_event(Interaction::PointerUp(point, PointingDevice::Mouse));
    }

    pub fn activate_tool(&mut self, tool: impl Tool + 'static) {
        self.active_tool = Some(Box::new(tool));
    }
}

impl<Id: Identifier> AddEntity for ViewPort<Id> {
    fn add_entity(&mut self, entity: Entity) {
        self.container.add_entity(entity);
    }
}

impl<Id: Identifier> Renderable for ViewPort<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        for entity in self.container.entities() {
            if let Some(render) = entity.query::<Render>() {
                (render.render)(entity, renderer);
            }
        }

        if let Some(active_tool) = &self.active_tool {
            active_tool.render(renderer);
        }
    }
}
