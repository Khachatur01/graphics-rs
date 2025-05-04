pub mod move_draw {
    use crate::entity::geometric::rectangle_entity::RectangleEntity;
    use core::entity::Entity;
    use geometry::figure::point::Point;
    use geometry::math::{Drag, Resize};

    pub fn mouse_down(entity: &mut Entity, current_point: &Point) {
        let rectangle: &mut RectangleEntity = entity.model_ref_mut();
        rectangle.rectangle.drag(current_point)
    }
    
    pub fn mouse_move(entity: &mut Entity, start: &Point, current_point: &Point) {
        let rectangle: &mut RectangleEntity = entity.model_ref_mut();
    
        let delta: Point = current_point - start;
    
        rectangle.rectangle.resize(delta.x(), delta.y());
    }
    
    pub fn mouse_up(entity: &mut Entity, start: &Point, current_point: &Point) {
        let rectangle: &mut RectangleEntity = entity.model_ref_mut();
    
        let delta: Point = current_point - start;
    
        rectangle.rectangle.resize(delta.x(), delta.y());
    }
}

pub mod render {
    use crate::entity::geometric::rectangle_entity::Entity;
    use crate::entity::geometric::rectangle_entity::RectangleEntity;
    use plugin_rendering::Renderer;

    pub fn render(entity: &Entity, renderer: &mut dyn Renderer) {
        let rectangle: &RectangleEntity = entity.model_ref();

        renderer.rectangle(rectangle.rectangle(), &rectangle.style);
    }
}
