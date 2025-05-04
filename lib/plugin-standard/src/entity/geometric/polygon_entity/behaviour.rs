use crate::behaviour::ClickDraw;
use crate::entity::geometric::polygon_entity::PolygonEntity;
use core::entity::Entity;
use plugin_rendering::behaviour::Render;

#[inline]
pub fn add_behaviours(entity: &mut Entity) {
    /* click draw */
    entity.add_behaviour(
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
    );

    /* render */
    entity.add_behaviour(
        Render {
            render: |entity, renderer| {
                let polygon: &PolygonEntity = entity.model_ref();

                renderer.polygon(polygon.polygon(), &polygon.style);
            }
        }
    );
}
