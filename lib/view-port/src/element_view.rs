use rendering::Render;

pub mod geometric;
pub mod text_view;

pub trait DefaultWithId<Id> {
    fn default_with_id(id: Id) -> Self;
}

pub trait ElementView<Id>: Render {
    fn id(&self) -> &Id;
}
