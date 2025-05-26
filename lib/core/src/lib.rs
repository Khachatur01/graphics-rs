use std::any::Any;
use dyn_clone::DynClone;

pub mod container;
pub mod entity;
pub mod feature_set;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Serialize {
    fn serialize(&self) -> Box<dyn Any> {
        Box::new(String::new())
    }
}

/* model trait */
pub trait Model: AsAny {}

/* feature trait */
pub trait Feature: AsAny {
    fn boxed(self) -> Box<dyn Feature>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
