mod render;

use element_view::ElementView;
use getter_methods::GetterMethods;
use std::any::Any;
use std::sync::{Arc, RwLock};

pub type AtomicVec<T> = Arc<RwLock<Vec<T>>>;
pub fn new_atomic_vec<T>() -> AtomicVec<T> {
    Arc::new(RwLock::new(vec![]))
}

#[derive(GetterMethods)]
pub struct ContainerView<Id> {
    id: Id,
    elements: AtomicVec<Box<dyn ElementView<Id>>>,
    behaviors: Vec<Box<dyn Any>>,
}

impl<Id: 'static> ContainerView<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            elements: new_atomic_vec(),
            behaviors: vec![],
        }
    }
}

impl<Id> ElementView<Id> for ContainerView<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
