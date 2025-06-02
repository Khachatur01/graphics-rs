use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use geometry::math::{Drag, Resize};
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use entity_model_feature::entity::Entity;
use entity_model_feature::{EntityId, Feature};
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature_derive::Model;
use standard_rendering_plugin::renderer::Renderer;
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_tool_plugin::MoveDraw;

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct RectangleModel {
    pub rectangle: Rectangle,
    pub style: ShapeStyle,
}

impl RectangleModel {
    pub fn entity(id: impl EntityId + 'static, rectangle: Rectangle, style: ShapeStyle) -> Entity {
        Entity::new(
            id,
            RectangleModel { rectangle, style },
            Self::standard_feature_set(),
        )
    }

    pub fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([
            Self::move_draw().boxed(),
            Self::render().boxed()
        ])
    }

    pub fn move_draw() -> MoveDraw {
        MoveDraw {
            mouse_down: |entity: &mut Entity, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();
                rectangle.rectangle.drag(current_point)
            },
            mouse_move: |entity: &mut Entity, start, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
            },
            mouse_up: |entity: &mut Entity, start, current_point| {
                let rectangle: &mut Self = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
            },
            finish: |entity: &mut Entity| {},
        }
    }

    pub fn render() -> Render {
        Render {
            render: |entity: &Entity, renderer: &mut dyn Renderer| {
                let rectangle: &Self = entity.model_ref();

                renderer.rectangle(rectangle.rectangle(), &rectangle.style);
            },
        }
    }
}
