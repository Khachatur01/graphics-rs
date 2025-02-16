use crate::tool::Tool;
use geometry::shape::point::Point;
use geometry::shape::Shape;
use view_port::view_port::element_id::ElementId;
use view_port::view_port::element_view::ElementView;
use view_port::view_port::traits::MouseEvents;
use view_port::view_port::ViewPort;

pub struct MoveDrawTool<'view_port, Drawable>
where Drawable: Shape + Copy + Clone + 'static {
    start_point: Option<Point>,
    drawable: Drawable,
    view_port: &'view_port mut ViewPort,
}

impl<'view_port, Drawable> MoveDrawTool<'view_port, Drawable>
where Drawable: Shape + Copy + Clone + 'static {
    pub fn new(view_port: &'view_port mut ViewPort, drawable: Drawable) -> Self {
        Self {
            start_point: None,
            drawable,
            view_port
        }
    }
}

impl<'view_port, Drawable> MouseEvents for MoveDrawTool<'_, Drawable>
where Drawable: Shape + Copy + Clone + 'static {
    fn mouse_down(&mut self, point: &Point) {
        self.start_point = Some(*point);
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(start_point) = self.start_point else {
            return;
        };

        let delta: Point = point - start_point;

        let width: f64 = delta.x();
        let height: f64 = delta.y();

        self.drawable.resize(width, height);
    }

    fn mouse_up(&mut self, point: &Point) {
        let Some(start_point) = self.start_point else {
            return;
        };

        let drawable: Box<Drawable> = Box::new(self.drawable.clone());

        let element_view: ElementView<ElementId> = ElementView::new(
            ElementId::new(0, 0),
            drawable
        );

        self.view_port.add_element(element_view);

        self.start_point = None;
    }
}

impl<'view_port, Drawable> Tool for MoveDrawTool<'_, Drawable>
where Drawable: Shape + Copy + Clone + 'static {}
