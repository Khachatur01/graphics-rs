pub mod click_draw {
    use crate::entity::geometric::polygon_entity::PolygonEntity;
    use core::entity::Entity;
    use geometry::figure::point::Point;

    pub fn mouse_down(entity: &mut Entity, current_point: &Point) {
        let polygon: &mut PolygonEntity = entity.model_ref_mut();

        polygon.polygon.add_vertex(current_point.clone());
        polygon.polygon.add_vertex(current_point.clone());
    }

    pub fn mouse_move(entity: &mut Entity, current_point: &Point) {
        let polygon: &mut PolygonEntity = entity.model_ref_mut();

        polygon.polygon.replace_last_vertex(current_point.clone());
    }

    pub fn mouse_up(_: &mut Entity, _: &Point) {}
}

pub mod render {
    use crate::entity::geometric::polygon_entity::PolygonEntity;
    use core::entity::Entity;
    use plugin_rendering::Renderer;

    pub fn render(entity: &Entity, renderer: &mut dyn Renderer) {
        let polygon: &PolygonEntity = entity.model_ref();

        renderer.polygon(polygon.polygon(), &polygon.style);
    }
}
