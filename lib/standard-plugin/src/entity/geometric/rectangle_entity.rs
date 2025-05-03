mod render;
mod draw;

use crate::entity::geometric::rectangle_entity::draw::{mouse_down, mouse_move, mouse_up};
use crate::entity::geometric::rectangle_entity::render::render;
use crate::interactivity::tool::draw_tool::MoveDraw;
use core::entity::Entity;
use core::entity::Model;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use rendering::behaviour::Render;
use rendering::style::shape_style::ShapeStyle;

#[derive(GetterMethods)]
pub struct RectangleEntity {
    rectangle: Rectangle,
    style: ShapeStyle,
}

impl Model for RectangleEntity {}

impl RectangleEntity {
    pub fn new<Id: 'static>(id: Id, rectangle: Rectangle, style: ShapeStyle) -> Entity<Id> {
        let mut entity = Entity::new(
            id,
            RectangleEntity {
                rectangle,
                style,
            }
        );

        entity.add_behaviour(Render {
            render: render::<Id>
        });
        entity.add_behaviour(MoveDraw {
            mouse_down: mouse_down::<Id>,
            mouse_move: mouse_move::<Id>,
            mouse_up: mouse_up::<Id>,
        });

        entity
    }
}
