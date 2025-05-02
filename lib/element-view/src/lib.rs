
pub trait DefaultWithId<Id> {
    fn default_with_id(id: Id) -> Self;
}

pub trait ElementView<Id> {
    fn id(&self) -> &Id;
}
