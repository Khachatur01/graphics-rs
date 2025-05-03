use crate::geometric::rectangle_view::RectangleElement;
use rendering::Renderer;
use std::any::Any;

pub fn render<Id: 'static>(element: &dyn Any, renderer: &mut dyn Renderer) {
    let rectangle: &RectangleElement = element.downcast_ref().unwrap();

    renderer.rectangle(rectangle.rectangle(), &rectangle.style);
}