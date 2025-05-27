use dyn_clone::DynClone;
use dyn_serde::Serialize;
use std::any::Any;
use std::fmt::Display;

pub mod container;
pub mod entity;
pub mod feature_set;

pub trait EntityId: Display + DynClone + AsSerialize {}

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait AsSerialize {
    fn as_serialize(&self) -> &dyn Serialize;
}

/* model trait */
pub trait Model: AsAny + AsSerialize {}

/* feature trait */
pub trait Feature: AsAny {
    fn boxed(self) -> Box<dyn Feature>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
