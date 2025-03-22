use rendering::Render;

pub mod geometric;
pub mod text_view;

pub trait ElementView<Id>: Render {
    fn id(&self) -> &Id;
}
