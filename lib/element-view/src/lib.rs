use std::any::Any;

pub trait DefaultWithId<Id> {
    fn default_with_id(id: Id) -> Self;
}

pub struct Element<Id, T: 'static> {
    id: Id,
    phantom_data: Option<T>,
    model: Box<dyn Any>,
    behaviours: Vec<Box<dyn Any>>,
}

impl<Id, T: 'static> Element<Id, T> {
    pub fn new(id: Id, model: T, behaviours: Vec<Box<dyn Any>>) -> Self {
        Self {
            id,
            phantom_data: None,
            model: Box::new(model),
            behaviours,
        }
    }
    pub fn downcast_ref(&self) -> Option<&T> {
        self.model.downcast_ref()
    }
    pub fn downcast_ref_mut(&mut self) -> Option<&mut T> {
        self.model.downcast_mut()
    }
}

pub trait ElementView<Id> {
    fn id(&self) -> &Id;
}

pub fn query<Id, El, T: Clone + 'static>(element: &Element<Id, El>) -> Vec<T> {
    element
        .behaviours
        .iter()
        .filter_map(|behaviour| {
            behaviour.downcast_ref::<T>().map(Clone::clone)
        })
        .collect()
}
