use core::entity::Entity;
use core::entity::Id;
use core_derive::Model;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::{Drag, Resize};
use getter_methods::GetterMethods;
use standard_rendering_plugin::renderer::Renderer;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_tool_plugin::MoveDraw;
use core::Feature;
use core::feature_set::FeatureSet;

#[derive(Model, GetterMethods)]
pub struct RectangleEntity {
    rectangle: Rectangle,
    style: ShapeStyle,
}

impl RectangleEntity {
    pub fn with_standard_feature_set(id: impl Id + 'static, rectangle: Rectangle, style: ShapeStyle) -> Entity {
        Entity::new(
            id,
            RectangleEntity {
                rectangle,
                style
            },
            Self::standard_feature_set()
        )
    }

    pub fn standard_feature_set() -> FeatureSet {
        FeatureSet::from(
            [
                Self::move_draw().boxed(),
                Self::render().boxed()
            ]
        )
    }

    pub fn move_draw() -> MoveDraw {
        MoveDraw {
            mouse_down: |entity, current_point| {
                let rectangle: &mut RectangleEntity = entity.model_ref_mut();
                rectangle.rectangle.drag(current_point)
            },
            mouse_move: |entity, start, current_point| {
                let rectangle: &mut RectangleEntity = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
            },
            mouse_up: |entity, start, current_point| {
                let rectangle: &mut RectangleEntity = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
            },
        }
    }

    pub fn render() -> Render {
        Render {
            render: |entity: &Entity, renderer: &mut dyn Renderer| {
                let rectangle: &RectangleEntity = entity.model_ref();

                renderer.rectangle(rectangle.rectangle(), &rectangle.style);
            },
        }
    }
}
