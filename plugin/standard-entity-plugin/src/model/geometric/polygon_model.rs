use crate::entity_model::{DefaultEntity, StandardFeatureSet};
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature::Feature;
use entity_model_feature_derive::Model;
use geometry::figure::polygon::Polygon;
use geometry::figure::rectangle::Rectangle;
use geometry::point::point_2d::Point2D;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_tool_plugin::{ClickDraw, Select};

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct PolygonModel {
    polygon: Polygon<Point2D>,
    style: ShapeStyle,
}

impl<Id: EntityId> DefaultEntity<Id> for PolygonModel {
    fn default_entity(id: Id) -> Entity<Id> {
        Entity::new(
            id,
            PolygonModel {
                polygon: Polygon::new(&[]),
                style: ShapeStyle::default(),
            },
            FeatureSet::empty(),
        )
    }
}

impl<Id: EntityId> StandardFeatureSet<Id> for PolygonModel {
    fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([
            Self::feature_click_draw::<Id>().boxed(),
            Self::feature_render::<Id>().boxed(),
            Self::feature_select::<Id>().boxed(),
        ])
    }
}

impl PolygonModel {
    pub fn feature_click_draw<Id: EntityId>() -> ClickDraw<Id> {
        ClickDraw {
            pointer_down: |entity, current_point| {
                let polygon: &mut PolygonModel = entity.model_ref_mut();

                polygon.polygon.add_vertex(current_point.clone());
                polygon.polygon.add_vertex(current_point.clone());
            },
            pointer_move: |entity, current_point| {
                let polygon: &mut PolygonModel = entity.model_ref_mut();

                polygon.polygon.replace_last_vertex(current_point.clone());
            },
            pointer_up: |_, _| {},
            finish: |entity| {
                let polygon: &mut PolygonModel = entity.model_ref_mut();
                polygon.polygon.remove_last_vertex();
            },
        }
    }

    pub fn feature_render<Id: EntityId>() -> Render<Id> {
        Render {
            render: |entity, renderer| {
                let polygon: &PolygonModel = entity.model_ref();

                renderer.polygon_2d(polygon.polygon(), &polygon.style);
            },
        }
    }

    pub fn feature_select<Id: EntityId>() -> Select<Id> {
        Select {
            select: |entity, selection: &Rectangle| {
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
