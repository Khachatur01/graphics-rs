pub mod geometric;
mod text_element;

pub trait ViewPortElement<Id> {
    fn id(&self) -> &Id;
}
