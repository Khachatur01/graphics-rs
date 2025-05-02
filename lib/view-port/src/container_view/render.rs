// use std::any::Any;
// use crate::element_view::container_view::ContainerView;
// use rendering::{Render, Renderer};

// impl<Id> Render for ContainerView<Id> {
//     fn render(&self, renderer: &mut dyn Renderer) {
//         let Ok(elements) = self.elements.read() else {
//             return; /* todo: add proper error propagation */
//         };
//
//         for element in elements.iter() {
//             element.render(renderer);
//         }
//     }
// }
