use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::path::Path;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use rendering::Renderer;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct CanvasRenderer {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d
}

#[wasm_bindgen]
impl CanvasRenderer {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self { canvas, context }
    }
}

impl Renderer for CanvasRenderer {
    fn clear(&mut self) {
        self.context.reset();
    }

    fn segment(&mut self, segment: &Segment) {
        self.context.begin_path();

        self.context.move_to(segment.start().x(), segment.start().y());
        self.context.line_to(segment.end().x(), segment.end().y());

        self.context.stroke();
    }

    fn rectangle(&mut self, rectangle: &Rectangle) {
        self.context.fill_rect(
            rectangle.top_left().x(),
            rectangle.top_left().y(),
            rectangle.width(),
            rectangle.height(),
        );

        self.context.stroke();
    }

    fn circle(&mut self, circle: &Circle) {
        todo!()
    }

    fn ellipse(&mut self, ellipse: &Ellipse) {
        todo!()
    }

    fn path(&mut self, path: &Path) {
        todo!()
    }
}
