mod render;
mod draw;

use crate::interactivity::tool::draw_tool::MoveDraw;
use core::entity::Entity;
use core::entity::Id;
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
    pub fn new(id: impl Id + 'static, rectangle: Rectangle, style: ShapeStyle) -> Entity {
        let mut entity = Entity::new(
            id,
            RectangleEntity {
                rectangle,
                style,
            }
        );

        entity.add_behaviour(Render {
            render: render::render
        });
        entity.add_behaviour(MoveDraw {
            mouse_down: draw::mouse_down,
            mouse_move: draw::mouse_move,
            mouse_up: draw::mouse_up,
        });

        entity
    }
}
