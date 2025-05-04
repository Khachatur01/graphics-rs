mod feature;

use crate::entity::geometric::rectangle_entity::feature::add_features;
use core::entity::Entity;
use core::entity::Id;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use core_derive::Model;
use standard_rendering_plugin::style::shape_style::ShapeStyle;

#[derive(Model, GetterMethods)]
pub struct RectangleEntity {
    rectangle: Rectangle,
    style: ShapeStyle,
}

impl RectangleEntity {
    pub fn new(id: impl Id + 'static, rectangle: Rectangle, style: ShapeStyle) -> Entity {
        let mut entity = Entity::new(
            id,
            RectangleEntity {
                rectangle,
                style,
            }
        );

        add_features(&mut entity);

        entity
    }
}
