mod draw;
mod render;

use crate::element_view::ElementView;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct TextView<Id> {
    id: Id,
    text: String,
    style: String,
}

impl<Id> TextView<Id> {
    pub fn new(id: Id, text: String, style: String) -> Self {
        Self { id, text, style }
    }
}

impl<Id> ElementView<Id> for TextView<Id> {
    fn id(&self) -> &Id {
        self.id()
    }
}
