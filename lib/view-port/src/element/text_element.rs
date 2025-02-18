mod draw;
mod render;

use crate::element::ViewPortElement;
use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct TextElement<Id> {
    id: Id,
    text: String,
    style: String,
}

impl<Id> TextElement<Id> {
    pub fn new(id: Id, text: String, style: String) -> Self {
        Self { id, text, style }
    }
}

impl<Id> ViewPortElement<Id> for TextElement<Id> {
    fn id(&self) -> &Id {
        self.id()
    }
}
