mod behaviour;

use crate::entity::geometric::rectangle_entity::behaviour::{move_draw_rectangle, render_rectangle};
use core::entity::Entity;
use core::entity::Id;
use core::entity::Model;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use plugin_rendering::style::shape_style::ShapeStyle;

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

        entity.add_behaviour(render_rectangle());
        entity.add_behaviour(move_draw_rectangle());

        entity
    }
}
