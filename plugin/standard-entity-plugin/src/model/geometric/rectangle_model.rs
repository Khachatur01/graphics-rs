use crate::entity_model::{DefaultEntity, StandardFeatureSet};
use algebra::linear::transformation::Transformations;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature::Feature;
use entity_model_feature_derive::Model;
use geometry::figure::rectangle::Rectangle;
use geometry::point::point_2d::Point2D;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use standard_rendering_plugin::renderer::renderer::Renderer;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_svg_plugin::property_map::PropertyMap;
use standard_svg_plugin::svg_element::svg_rectangle::SVGRectangle;
use standard_svg_plugin::svg_element::{SVGElement, SVG};
use standard_svg_plugin::ToSVG;
use standard_tool_plugin::{MoveDraw, Transform};

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct RectangleModel {
    pub rectangle: Rectangle,
    pub style: ShapeStyle,

    pub transformations: Transformations
}

impl<Id: EntityId> DefaultEntity<Id> for RectangleModel {
    fn default_entity(id: Id) -> Entity<Id> {
        Entity::new(
            id,
            RectangleModel {
                rectangle: Rectangle::zero_sized(Point2D::default()),
                style: ShapeStyle::default(),
                transformations: Transformations::empty()
            },
            FeatureSet::empty(),
        )
    }
}

impl<Id: EntityId> StandardFeatureSet<Id> for RectangleModel {
    fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([
            Self::feature_move_draw::<Id>().boxed(),
            Self::feature_transform::<Id>().boxed(),
            Self::feature_to_svg::<Id>().boxed(),
            Self::feature_render::<Id>().boxed()
        ])
    }
}

impl RectangleModel {
    pub fn feature_move_draw<Id: EntityId>() -> MoveDraw<Id> {
        MoveDraw {
            pointer_down: |entity, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();
                rectangle.rectangle.set_top_left(*current_point)
            },
            pointer_move: |entity, start, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();

                let delta: Point2D = current_point - start;
                rectangle.rectangle.set_size(delta.x, delta.y);
            },
            pointer_up: |entity, start, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();

                let delta: Point2D = current_point - start;
                rectangle.rectangle.set_size(delta.x, delta.y);
            },
            finish: |entity| {},
        }
    }

    pub fn feature_transform<Id: EntityId>() -> Transform<Id> {
        Transform {
            transform: |entity, transformation| {
                let rectangle: &mut Self = entity.model_ref_mut();

                rectangle.transformations.push(transformation.clone());
            }
        }
    }

    pub fn feature_to_svg<Id: EntityId>() -> ToSVG<Id> {
        ToSVG {
            to_svg: |entity| -> SVGElement {
                let rectangle: &Self = entity.model_ref();

                SVGElement::new(
                    SVG::Rectangle(SVGRectangle {
                        x: rectangle.rectangle.top_left.x,
                        y: rectangle.rectangle.top_left.y,
                        width: rectangle.rectangle.width,
                        height: rectangle.rectangle.height,
                    }),
                    rectangle.transformations.compose(),
                    PropertyMap::from([
                        /* todo */
                    ]),
                    PropertyMap::from([
                        /* todo */
                    ])
                )
            }
        }
    }

    pub fn feature_render<Id: EntityId>() -> Render<Id> {
        Render {
            render: |entity, renderer: &mut dyn Renderer| {
                let rectangle: &Self = entity.model_ref();

                renderer.rectangle(
                    rectangle.rectangle(),
                    &rectangle.style,
                    rectangle.transformations.compose()
                );
            },
        }
    }
}
