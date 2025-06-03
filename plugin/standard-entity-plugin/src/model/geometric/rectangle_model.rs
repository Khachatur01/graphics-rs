use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature::Feature;
use entity_model_feature_derive::Model;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::{Drag, Resize};
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use standard_rendering_plugin::renderer::Renderer;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_svg_plugin::property_map::PropertyMap;
use standard_svg_plugin::svg_element::svg_rectangle::SVGRectangle;
use standard_svg_plugin::svg_element::SVGElement;
use standard_svg_plugin::ToSVG;
use standard_tool_plugin::MoveDraw;

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct RectangleModel {
    pub rectangle: Rectangle,
    pub style: ShapeStyle,
}

impl RectangleModel {
    pub fn entity<Id: EntityId>(id: Id, rectangle: Rectangle, style: ShapeStyle) -> Entity<Id> {
        Entity::new(
            id,
            RectangleModel { rectangle, style },
            Self::standard_feature_set::<Id>(),
        )
    }

    pub fn standard_feature_set<Id: EntityId>() -> FeatureSet {
        FeatureSet::from([
            Self::feature_move_draw::<Id>().boxed(),
            Self::feature_to_svg::<Id>().boxed(),
            Self::feature_render::<Id>().boxed()
        ])
    }

    pub fn feature_move_draw<Id: EntityId>() -> MoveDraw<Id> {
        MoveDraw {
            mouse_down: |entity, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();
                rectangle.rectangle.drag(current_point)
            },
            mouse_move: |entity, start, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
            },
            mouse_up: |entity, start, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
            },
            finish: |entity| {},
        }
    }

    pub fn feature_to_svg<Id: EntityId>() -> ToSVG<Id> {
        ToSVG {
            to_svg: |entity| -> SVGElement {
                let rectangle: &Self = entity.model_ref();

                SVGElement::rectangle(
                    SVGRectangle {
                        x: rectangle.rectangle.top_left().x,
                        y: rectangle.rectangle.top_left().y,
                        width: rectangle.rectangle.width(),
                        height: rectangle.rectangle.height(),
                    },
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

                renderer.rectangle(rectangle.rectangle(), &rectangle.style);
            },
        }
    }
}
