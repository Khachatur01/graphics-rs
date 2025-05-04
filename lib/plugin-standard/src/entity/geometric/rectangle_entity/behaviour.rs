use crate::behaviour::MoveDraw;
use crate::entity::geometric::rectangle_entity::RectangleEntity;
use core::entity::Entity;
use geometry::figure::point::Point;
use geometry::math::{Drag, Resize};
use plugin_rendering::behaviour::Render;
use plugin_rendering::Renderer;

#[inline]
pub fn move_draw_rectangle() -> MoveDraw {
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

#[inline]
pub fn render_rectangle() -> Render {
    Render {
        render: |entity: &Entity, renderer: &mut dyn Renderer| {
            let rectangle: &RectangleEntity = entity.model_ref();

            renderer.rectangle(rectangle.rectangle(), &rectangle.style);
        },
    }
}
