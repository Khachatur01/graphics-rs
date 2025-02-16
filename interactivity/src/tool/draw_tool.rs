use crate::tool::Tool;
use geometry::shape::point::Point;
use geometry::shape::Shape;
use geometry::traits::{Drag, Resize};
use view_port::view_port::element_id::ElementId;
use view_port::view_port::element_view::ElementView;
use view_port::view_port::traits::MouseEvents;
use view_port::view_port::ViewPort;

pub struct DrawTool<'view_port> {
    start_point: Option<Point>,
    drawable: Shape,
    view_port: &'view_port mut ViewPort,
}

impl<'view_port> DrawTool<'view_port> {
    pub fn new(view_port: &'view_port mut ViewPort, drawable: Shape) -> Self {
        Self {
            start_point: None,
            drawable,
            view_port
        }
    }
}

impl<'view_port> MouseEvents for DrawTool<'view_port> {
    fn mouse_down(&mut self, point: &Point) {
        self.start_point = Some(*point);

        match &mut self.drawable {
            Shape::Point(point) => {}
            Shape::Segment(segment) => {}
            Shape::Rectangle(rectangle) => {
                rectangle.drag(point)
            },
            Shape::Circle(circle) => {

            }
        }

        todo!()
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(start_point) = self.start_point else {
            return;
        };

        match &mut self.drawable {
            Shape::Point(point) => {}
            Shape::Segment(segment) => {}
            Shape::Rectangle(rectangle) => {
                let width: f64 = point.x();
                let height: f64 = point.x();
                rectangle.resize(width, height);
            },
            Shape::Circle(circle) => {

            }
        }

        todo!()
    }

    fn mouse_up(&mut self, point: &Point) {
        let Some(start_point) = self.start_point else {
            return;
        };

        match &mut self.drawable {
            Shape::Point(point) => {}
            Shape::Segment(segment) => {}
            Shape::Rectangle(rectangle) => {
                let element_view: ElementView<ElementId> = ElementView::from_shape(
                    ElementId::new(0, 0),
                    Shape::Rectangle(rectangle.clone()),
                );

                self.view_port.add_element(element_view);
            },
            Shape::Circle(circle) => {
                let element_view: ElementView<ElementId> = ElementView::from_shape(
                    ElementId::new(0, 0),
                    Shape::Circle(circle.clone()),
                );

                self.view_port.add_element(element_view);
            }
        }

        todo!()
    }
}

impl<'view_port> Tool for DrawTool<'view_port> {}
