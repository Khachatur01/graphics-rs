use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use rendering::style::shape_style::ShapeStyle;
use rendering::Renderer;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct CanvasRenderer {
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

        Self { context }
    }
}

impl CanvasRenderer {
    fn apply_style(&mut self, style: &ShapeStyle) {
        self.context.set_fill_style_str(&style.fill_color.to_hex());
        self.context.set_stroke_style_str(&style.stroke_color.to_hex());
        self.context.set_line_width(style.stroke_width);
    }
}

impl Renderer for CanvasRenderer {
    fn clear(&mut self) {
        self.context.reset();
    }

    fn segment(&mut self, segment: &Segment, style: &ShapeStyle) {
        self.apply_style(style);

        self.context.begin_path();

        self.context.move_to(segment.start().x(), segment.start().y());
        self.context.line_to(segment.end().x(), segment.end().y());

        self.context.stroke();
    }

    fn rectangle(&mut self, rectangle: &Rectangle, style: &ShapeStyle) {
        self.apply_style(style);

        self.context.begin_path();

        self.context.fill_rect(
            rectangle.top_left().x(),
            rectangle.top_left().y(),
            rectangle.width(),
            rectangle.height(),
        );
        self.context.rect(
            rectangle.top_left().x(),
            rectangle.top_left().y(),
            rectangle.width(),
            rectangle.height(),
        );

        self.context.stroke();
    }

    fn circle(&mut self, circle: &Circle, style: &ShapeStyle) {
        self.apply_style(style);

        todo!()
    }

    fn ellipse(&mut self, ellipse: &Ellipse, style: &ShapeStyle) {
        self.apply_style(style);

        todo!()
    }
}
