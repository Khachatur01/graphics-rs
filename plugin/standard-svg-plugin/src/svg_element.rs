use crate::styles::SVGStyle;
use crate::svg_element::svg_circle::SVGCircle;
use crate::svg_element::svg_line::SVGLine;
use crate::svg_element::svg_rectangle::SVGRectangle;

pub mod svg_circle;
pub mod svg_line;
pub mod svg_rectangle;

pub trait SVGElement1 {
    fn set_property(&mut self, key: &'static str, value: &'static str);
    fn unset_property(&mut self, key: &'static str);
}

pub struct SVGElement {
    svg_type: SVGType,
    style: SVGStyle,
}

impl SVGElement {
    pub fn new(svg_type: SVGType, style: SVGStyle) -> Self {
        Self {
            svg_type,
            style
        }
    }

    pub fn circle(svg_circle: SVGCircle, style: SVGStyle) -> SVGElement {
        Self {
            svg_type: SVGType::Circle(svg_circle),
            style
        }
    }

    pub fn rectangle(svg_rectangle: SVGRectangle, style: SVGStyle) -> SVGElement {
        Self {
            svg_type: SVGType::Rectangle(svg_rectangle),
            style
        }
    }
}

pub enum SVGType {
    Circle(SVGCircle),
    Ellipse,
    Line(SVGLine),
    Polygon,
    Polyline,
    Rectangle(SVGRectangle),
    Text,
    Image,
    Path,
    ForeignObject,
    Group,
}
