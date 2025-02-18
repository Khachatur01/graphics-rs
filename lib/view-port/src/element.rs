pub mod geometric_element;
mod text_element;

pub trait ViewPortElement<Id> {
    fn id(&self) -> &Id;
}
