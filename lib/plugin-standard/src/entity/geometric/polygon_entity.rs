mod behaviour;

use crate::entity::geometric::polygon_entity::behaviour::{click_draw_polygon, render_polygon};
use core::entity::Entity;
use core::entity::Id;
use core::entity::Model;
use geometry::figure::polygon::Polygon;
use getter_methods::GetterMethods;
use plugin_rendering::style::shape_style::ShapeStyle;

#[derive(GetterMethods)]
pub struct PolygonEntity {
    polygon: Polygon,
    style: ShapeStyle,
}

impl Model for PolygonEntity {}

impl PolygonEntity {
    pub fn new(id: impl Id + 'static, polygon: Polygon, style: ShapeStyle) -> Entity {
        let mut entity = Entity::new(
            id,
            PolygonEntity {
                polygon,
                style,
            }
        );

        entity.add_behaviour(render_polygon());
        entity.add_behaviour(click_draw_polygon());

        entity
    }
}
