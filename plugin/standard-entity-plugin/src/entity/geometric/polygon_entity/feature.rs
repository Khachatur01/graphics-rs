use crate::entity::geometric::polygon_entity::PolygonEntity;
use core::entity::Entity;
use standard_rendering_plugin::Render;
use standard_tool_plugin::ClickDraw;

#[inline]
pub fn add_features(entity: &mut Entity) {
    entity.add_feature(
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

    entity.add_feature(
        Render {
            render: |entity, renderer| {
                let polygon: &PolygonEntity = entity.model_ref();

                renderer.polygon(polygon.polygon(), &polygon.style);
            }
        }
    );
}
