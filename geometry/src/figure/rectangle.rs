use crate::figure::segment::Segment;
use algebra::linear::vector::Vector;

#[derive(Clone)]
pub struct Rectangle {
    pub top_left: Vector<2>,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(top_left: Vector<2>, width: f64, height: f64) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }

    pub fn zero_sized(top_left: Vector<2>) -> Self {
        Self {
            top_left,
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn absolute_sized(&self) -> Self {
        // let mut rectangle_clone: Rectangle = self.clone();
        //
        // if rectangle_clone.width < 0.0 {
        //     rectangle_clone.width *= -1.0;
        //     rectangle_clone.top_left.x() -= rectangle_clone.width;
        // }
        // if rectangle_clone.height < 0.0 {
        //     rectangle_clone.height *= -1.0;
        //     rectangle_clone.top_left.y() -= rectangle_clone.height;
        // }
        //
        // rectangle_clone

        todo!()
    }

    pub fn set_top_left(&mut self, point: Vector<2>) {
        self.top_left = point;
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

impl From<&Rectangle> for [Segment<2>; 4] {
    fn from(rectangle: &Rectangle) -> [Segment<2>; 4] {
        // let top_left = rectangle.top_left;
        // let top_right = top_left + Vector::<2>::new(rectangle.width, 0.0);
        // let bottom_right = top_right + Vector::<2>::new(0.0, rectangle.height);
        // let bottom_left = bottom_right - Vector::<2>::new(rectangle.width, 0.0);
        //
        // [
        //     Segment::new(top_left, top_right),
        //     Segment::new(top_right, bottom_right),
        //     Segment::new(bottom_right, bottom_left),
        //     Segment::new(bottom_left, top_left),
        // ]
        todo!()
    }
}
