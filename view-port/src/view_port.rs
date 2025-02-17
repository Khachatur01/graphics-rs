use crate::view_port::element_view::ElementView;
use getter_methods::GetterMethods;
use std::sync::{Arc, RwLock};

pub mod traits;
pub mod element_view;
pub mod element;


/* todo: move to separate module */
pub type AtomicRW<T> = Arc<RwLock<T>>;
pub fn new_atomic_rw<T>(value: T) -> AtomicRW<T> {
    Arc::new(RwLock::new(value))
}

#[derive(GetterMethods)]
pub struct ViewPort {
    elements: AtomicRW<Vec<ElementView>>,
}

impl ViewPort {
    pub fn new() -> Self {
        Self {
            elements: new_atomic_rw(vec![]),
        }
    }

    pub fn add_element(&self, element: ElementView) -> Result<(), ()> {
        let Ok(mut elements) = self.elements.write() else {
            return Err(());
        };

        elements.push(element);

        Ok(())
    }
}
