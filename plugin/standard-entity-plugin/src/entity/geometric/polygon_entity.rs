mod feature;

use crate::entity::geometric::polygon_entity::feature::add_features;
use core::entity::Entity;
use core::entity::Id;
use geometry::figure::polygon::Polygon;
use getter_methods::GetterMethods;
use core_derive::Model;
use standard_rendering_plugin::style::shape_style::ShapeStyle;

#[derive(Model, GetterMethods)]
pub struct PolygonEntity {
    polygon: Polygon,
    style: ShapeStyle,
}

impl PolygonEntity {
    pub fn new(id: impl Id + 'static, polygon: Polygon, style: ShapeStyle) -> Entity {
        let mut entity = Entity::new(
            id,
            PolygonEntity {
                polygon,
                style,
            }
        );

        add_features(&mut entity);

        entity
    }
}
