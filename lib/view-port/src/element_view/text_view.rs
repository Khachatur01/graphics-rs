mod draw;
mod render;

use crate::element_view::ElementView;
use getter_methods::GetterMethods;
use rendering::style::text_style::TextStyle;

#[derive(GetterMethods)]
pub struct TextView<Id> {
    id: Id,
    text: String,
    style: TextStyle,
}

impl<Id> TextView<Id> {
    pub fn new(id: Id, text: String) -> Self {
        Self {
            id,
            text,
            style: TextStyle::default(),
        }
    }
}

impl<Id> ElementView<Id> for TextView<Id> {
    fn id(&self) -> &Id {
        self.id()
    }
}
