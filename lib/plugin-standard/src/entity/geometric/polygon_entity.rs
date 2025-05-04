mod behaviour;

use crate::behaviour::ClickDraw;
use crate::entity::geometric::polygon_entity::behaviour::{click_draw, render};
use core::entity::Entity;
use core::entity::Id;
use core::entity::Model;
use geometry::figure::polygon::Polygon;
use getter_methods::GetterMethods;
use plugin_rendering::behaviour::Render;
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

        entity.add_behaviour(Render {
            render: render::render
        });
        entity.add_behaviour(ClickDraw {
            mouse_down: click_draw::mouse_down,
            mouse_move: click_draw::mouse_move,
            mouse_up: click_draw::mouse_up,
        });

        entity
    }
}
