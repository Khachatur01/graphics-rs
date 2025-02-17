use crate::tool::draw_tool::DrawTool;
use geometry::figure::point::Point;
use view_port::view_port::element::Element;
use view_port::view_port::ViewPort;

pub mod tool;

pub fn a() {
    let vp = ViewPort::new();

    let shape = Element::Point(Point::new(0.0, 0.0));
    let dt = DrawTool::new(&vp, shape);

    let shape = Element::Point(Point::new(0.0, 0.0));
    let dt = DrawTool::new(&vp, shape);
}
