use std::any::{Any};

pub mod entity;
pub mod container;
pub mod feature_set;


pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/* model trait */
pub trait Model: AsAny {}

/* feature trait */
pub trait Feature: AsAny {
    fn boxed(self) -> Box<dyn Feature>
    where Self: Sized {
        Box::new(self)
    }
}
