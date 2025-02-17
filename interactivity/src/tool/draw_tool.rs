use crate::tool::Tool;
use geometry::figure::point::Point;
use view_port::view_port::element::Element;
use view_port::view_port::traits::MouseEvents;
use view_port::view_port::ViewPort;

pub struct DrawTool<'vp> {
    start_point: Option<Point>,
    drawable: Element,
    view_port: &'vp ViewPort,
}

impl<'vp> DrawTool<'vp> {
    pub fn new(view_port: &'vp ViewPort, drawable: Element) -> Self {
        Self {
            start_point: None,
            drawable,
            view_port
        }
    }
}

impl<'vp> MouseEvents for DrawTool<'vp> {
    fn mouse_down(&mut self, point: &Point) {
        self.start_point = Some(*point);
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(start_point) = self.start_point else {
            return;
        };

        let width: f64 = point.x() - start_point.x();
        let height: f64 = point.y() - start_point.y();
    }

    fn mouse_up(&mut self, point: &Point) {
        let Some(start_point) = self.start_point else {
            return;
        };

        self.start_point = None;
    }
}

impl<'vp> Tool for DrawTool<'vp> {}
