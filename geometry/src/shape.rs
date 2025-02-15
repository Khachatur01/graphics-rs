use crate::traits::{Drag, Resize};

pub mod point;
pub mod segment;
pub mod rectangle;
pub mod circle;
pub mod ellipse;
pub mod polygon;
pub mod polyline;
pub mod mesh;
pub mod path;


pub trait Shape: Resize + Drag {}
