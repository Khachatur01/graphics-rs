use crate::view_port::element::Element;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct ElementId {
    owner_id: usize,
    index: usize,
}

impl ElementId {
    pub fn new(owner_id: usize, index: usize) -> Self {
        Self { owner_id, index }
    }
}


#[derive(GetterMethods)]
pub struct ElementView {
    id: ElementId,
    element: Element,
    style: String,
}

impl ElementView {
    pub fn new(id: ElementId, element: Element) -> Self {
        Self {
            id,
            element,
            style: String::from(""),
        }
    }
}
