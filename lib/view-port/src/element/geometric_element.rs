mod draw;
mod render;

use crate::element::ViewPortElement;
use geometry::figure::point::Point;
use geometry::figure::segment::Segment;
use getter_methods::GetterMethods;

pub enum GeometricFigure {
    Point(Point),
    Segment(Segment),
}

#[derive(GetterMethods)]
pub struct GeometricElement<Id> {
    id: Id,
    geometric_figure: GeometricFigure,
    style: String,
}

impl<Id> GeometricElement<Id> {
    pub fn new(id: Id, geometric_figure: GeometricFigure) -> Self {
        Self {
            id,
            geometric_figure,
            style: String::from(""),
        }
    }
}

impl<Id> ViewPortElement<Id> for GeometricElement<Id> {
    fn id(&self) -> &Id {
        self.id()
    }
}
