use std::any::Any;
use dyn_serde::Serialize;

pub mod container;
pub mod entity;
pub mod feature_set;

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
