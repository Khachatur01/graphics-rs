use crate::entity::geometric::rectangle_entity::RectangleEntity;
use core::entity::Entity;
use geometry::figure::point::Point;
use geometry::math::{Drag, Resize};
use standard_rendering_plugin::Render;
use standard_rendering_plugin::renderer::Renderer;
use standard_tool_plugin::MoveDraw;

#[inline]
pub fn add_features(entity: &mut Entity) {
    entity.add_feature(
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
    );
    entity.add_feature(
        Render {
            render: |entity: &Entity, renderer: &mut dyn Renderer| {
                let rectangle: &RectangleEntity = entity.model_ref();

                renderer.rectangle(rectangle.rectangle(), &rectangle.style);
            },
        }
    );
}
