use crate::figure::point::Point;
use crate::figure::rectangle::Rectangle;

pub trait Resize {
    fn resize(&mut self, new_width: f64, new_height: f64);
}

pub trait Drag {
    fn drag(&mut self, delta: &Point);
}

pub trait Selectable {
    fn select(&mut self, selection: Rectangle) -> bool;
}

pub trait BoundingBox {
    fn bounding_box(&self) -> Rectangle;
}

pub trait AddPoint {
    fn add_point(&mut self, index: usize, point: Point);
}

pub trait RemovePoint {
    fn remove_point(&mut self, index: usize);
}

pub trait DragPoint {
    fn drag_point(&mut self, index: usize, delta: &Point);
}
