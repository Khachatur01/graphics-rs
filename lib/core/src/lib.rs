use std::any::{Any, TypeId};

pub mod entity;
pub mod interactivity;
pub mod container;
pub mod feature_set;

pub trait Model: Any {}

impl dyn Model {
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.downcast_mut::<T>()
    }
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
    }
}

pub trait Feature: Any {
    fn boxed(self) -> Box<dyn Feature>
    where Self: Sized {
        Box::new(self)
    }
}

impl dyn Feature {
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.downcast_mut::<T>()
    }
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
    }
}