use crate::svg_element::svg_circle::SVGCircle;

mod svg_circle;

pub enum SVGElement {
    Circle(SVGCircle),
    Ellipse,
    Line,
    Polygon,
    Polyline,
    Rectangle,
    Text,
    Image,
    Path,
    ForeignObject,
    Group,
}
