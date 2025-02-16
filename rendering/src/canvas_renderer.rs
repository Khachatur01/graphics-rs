use crate::Renderer;
use geometry::shape::point::Point;
use geometry::shape::Shape;
use view_port::view_port::ViewPort;

pub trait Canvas {
    fn rect(&mut self, point: &Point, width: f64, height: f64);
}

pub struct CanvasRenderer<BitmapCanvas> {
    canvas: BitmapCanvas
}

impl<BitmapCanvas: Canvas> CanvasRenderer<BitmapCanvas> {
    pub fn new(canvas: BitmapCanvas) -> Self {
        Self { canvas }
    }
}

impl<BitmapCanvas: Canvas> Renderer for CanvasRenderer<BitmapCanvas> {
    fn render(&mut self, view_port: &ViewPort) {
        view_port.elements().iter().for_each(|element| {
            match element.shape() {
                Shape::Point(_) => {}
                Shape::Segment(_) => {}
                Shape::Rectangle(rectangle) => {
                    self.canvas.rect(rectangle.top_left(), rectangle.width(), rectangle.height());
                }
                Shape::Circle(_) => {}
            }
        });
    }
}
