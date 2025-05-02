mod draw;
mod render;

use element_view::ElementView;
use getter_methods::GetterMethods;
use rendering::style::text_style::TextStyle;
use std::any::Any;

#[derive(GetterMethods)]
pub struct TextView<Id> {
    id: Id,
    text: String,
    style: TextStyle,
    behaviors: Vec<Box<dyn Any>>,
}

impl<Id> TextView<Id> {
    pub fn new(id: Id, text: String) -> Self {
        Self {
            id,
            text,
            style: TextStyle::default(),
            behaviors: vec![],
        }
    }
}

impl<Id> ElementView<Id> for TextView<Id> {
    fn id(&self) -> &Id {
        self.id()
    }
}
