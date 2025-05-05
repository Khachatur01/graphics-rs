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

	entity.add_feature(Self::click_draw());
	entity.add_feature(Self::render());
	entity.add_feature(Self::select());

        entity
    }

    pub fn click_draw() -> ClickDraw {
	ClickDraw {
            mouse_down: |entity, current_point| {
                let polygon: &mut PolygonEntity = entity.model_ref_mut();

                polygon.polygon.add_vertex(current_point.clone());
                polygon.polygon.add_vertex(current_point.clone());
            },
            mouse_move: |entity, current_point| {
                let polygon: &mut PolygonEntity = entity.model_ref_mut();

                polygon.polygon.replace_last_vertex(current_point.clone());
            },
            mouse_up: |_, _| {

            },
        }
    }

    pub fn render() -> Render {
	Render {
            render: |entity, renderer| {
                let polygon: &PolygonEntity = entity.model_ref();

                renderer.polygon(polygon.polygon(), &polygon.style);
            }
        }
    }

    pub fn select() -> Select {
	Select {
            select: |entity: &Entity, selection: &Rectangle| {
                let polygon: &PolygonEntity = entity.model_ref();

                false
                // match *selection {
                //     Selection::FullInclusion(rectangle) => polygon.polygon.is_inside_rectangle(rectangle),
                //     Selection::Intersection(rectangle) => polygon.polygon.intersects_rectangle(rectangle) || polygon.polygon.is_inside_rectangle(rectangle),
                // }
            }
        }
    }
}
