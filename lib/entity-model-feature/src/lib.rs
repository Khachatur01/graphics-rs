use dyn_clone::DynClone;
use dyn_serde::Serialize;
use std::any::Any;
use std::fmt::Display;

pub mod container;
pub mod entity;
pub mod feature_set;

pub trait EntityId: Display + AsSerialize + DynClone {}

impl Clone for Box<dyn EntityId> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait AsSerialize {
    fn as_serialize(&self) -> &dyn Serialize;
}

/* model trait */
pub trait Model: AsAny + AsSerialize + DynClone {}

impl Clone for Box<dyn Model> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}


/* feature trait */
pub trait Feature: AsAny + DynClone {
    fn boxed(self) -> Box<dyn Feature>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl Clone for Box<dyn Feature> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}
