use crate::view_port::element_id::ElementId;
use crate::view_port::element_view::ElementView;

pub mod element_view;
pub mod element_id;
pub mod traits;

pub struct ViewPort {
    elements: Vec<ElementView<ElementId>>,
}

impl ViewPort {
    pub fn new() -> Self {
        Self {
            elements: vec![],
        }
    }

    pub fn add_element(&mut self, element: ElementView<ElementId>) {
        self.elements.push(element);
    }
}