use crate::element::ViewPortElement;
use getter_methods::GetterMethods;
use rendering::{Render, Renderer};
use std::sync::{Arc, RwLock};

pub mod element;


pub type AtomicVec<T> = Arc<RwLock<Vec<T>>>;
pub fn new_atomic_vec<T>() -> AtomicVec<T> {
    Arc::new(RwLock::new(vec![]))
}

#[derive(GetterMethods)]
pub struct ViewPort<Id> {
    elements: AtomicVec<Box<dyn ViewPortElement<Id>>>,
}

impl<Id> ViewPort<Id> {
    pub fn new() -> Self {
        Self {
            elements: new_atomic_vec(),
        }
    }

    pub fn add_element(&self, element: impl ViewPortElement<Id> + 'static) -> Result<(), ()> {
        let Ok(mut elements) = self.elements.write() else {
            return Err(());
        };

        elements.push(Box::new(element));

        Ok(())
    }
}

impl<Id> Render for ViewPort<Id> {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Ok(elements) = self.elements.read() else {
            return; /* todo: add proper error propagation */
        };

        for element in elements.iter() {
            element.render(renderer);
        }
    }
}
