use core::entity::Entity;
use core::entity::Identifier;
use core::feature_set::FeatureSet;
use core::Feature;
use core_derive::Model;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use serde::Serialize;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_tool_plugin::{ClickDraw, Select};

#[derive(Model, Serialize, Clone, GetterMethods)]
pub struct PolygonEntity {
    polygon: Polygon,
    style: ShapeStyle,
}

impl PolygonEntity {
    pub fn with_standard_feature_set(
        id: impl Identifier + 'static,
        polygon: Polygon,
        style: ShapeStyle,
    ) -> Entity {
        Entity::new(
            id,
            PolygonEntity { polygon, style },
            Self::standard_feature_set(),
        )
    }

    pub fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([
            Self::click_draw().boxed(),
            Self::render().boxed(),
            Self::select().boxed(),
        ])
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
            mouse_up: |_, _| {},
        }
    }

    pub fn render() -> Render {
        Render {
            render: |entity, renderer| {
                let polygon: &PolygonEntity = entity.model_ref();

                renderer.polygon(polygon.polygon(), &polygon.style);
            },
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
            },
        }
    }
}
