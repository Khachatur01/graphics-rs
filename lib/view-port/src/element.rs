use rendering::Render;

pub mod geometric;
mod text_element;

pub trait ViewPortElement<Id>: Render {
    fn id(&self) -> &Id;
}
