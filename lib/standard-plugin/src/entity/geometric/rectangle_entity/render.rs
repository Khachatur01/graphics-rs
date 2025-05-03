use crate::entity::geometric::rectangle_entity::RectangleEntity;
use core::entity::Entity;
use rendering::Renderer;

pub fn render<Id>(entity: &Entity<Id>, renderer: &mut dyn Renderer) {
    let rectangle: &RectangleEntity = entity.model_ref();

    renderer.rectangle(rectangle.rectangle(), &rectangle.style);
}
