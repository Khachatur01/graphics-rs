mod render;

use crate::element_view::ElementView;
use getter_methods::GetterMethods;
use std::sync::{Arc, RwLock};

pub type AtomicVec<T> = Arc<RwLock<Vec<T>>>;
pub fn new_atomic_vec<T>() -> AtomicVec<T> {
    Arc::new(RwLock::new(vec![]))
}

#[derive(GetterMethods)]
pub struct ContainerView<Id> {
    id: Id,
    elements: AtomicVec<Box<dyn ElementView<Id>>>,
}

impl<Id> ContainerView<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            elements: new_atomic_vec(),
        }
    }

    pub fn add_element(&self, element: impl ElementView<Id> + 'static) -> Result<(), ()> {
        let Ok(mut elements) = self.elements.write() else {
            return Err(());
        };

        elements.push(Box::new(element));

        Ok(())
    }
}

impl<Id> ElementView<Id> for ContainerView<Id> {
    fn id(&self) -> &Id {
        &self.id
    }
}
