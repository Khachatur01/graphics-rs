use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use entity_model_feature::entity::Entity;
use entity_model_feature::{EntityId, Feature};
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature_derive::Model;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_tool_plugin::{ClickDraw, Select};

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct PolygonModel {
    polygon: Polygon,
    style: ShapeStyle,
}

impl PolygonModel {
    pub fn entity(
        id: impl EntityId + 'static,
        polygon: Polygon,
        style: ShapeStyle,
    ) -> Entity {
        Entity::new(
            id,
            PolygonModel { polygon, style },
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
                let polygon: &mut PolygonModel = entity.model_ref_mut();

                polygon.polygon.add_vertex(current_point.clone());
                polygon.polygon.add_vertex(current_point.clone());
            },
            mouse_move: |entity, current_point| {
                let polygon: &mut PolygonModel = entity.model_ref_mut();

                polygon.polygon.replace_last_vertex(current_point.clone());
            },
            mouse_up: |_, _| {},
            finish: |entity: &mut Entity| {
                let polygon: &mut PolygonModel = entity.model_ref_mut();
                polygon.polygon.remove_last_vertex();
            },
        }
    }

    pub fn render() -> Render {
        Render {
            render: |entity, renderer| {
                let polygon: &PolygonModel = entity.model_ref();

                renderer.polygon(polygon.polygon(), &polygon.style);
            },
        }
    }

    pub fn select() -> Select {
        Select {
            select: |entity: &Entity, selection: &Rectangle| {
                let polygon: &PolygonModel = entity.model_ref();

                false
                // match *selection {
                //     Selection::FullInclusion(rectangle) => polygon.polygon.is_inside_rectangle(rectangle),
                //     Selection::Intersection(rectangle) => polygon.polygon.intersects_rectangle(rectangle) || polygon.polygon.is_inside_rectangle(rectangle),
                // }
            },
        }
    }
}
