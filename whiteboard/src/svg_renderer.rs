use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::path::Path;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;
use rendering::Renderer;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Node, SvgElement, SvgLineElement, SvgRectElement};

#[wasm_bindgen]
pub struct SVGRenderer {
    svg: SvgElement,
}

#[wasm_bindgen]
impl SVGRenderer {
    pub fn new(svg: SvgElement) -> Self {
        Self { svg }
    }
}

impl Renderer for SVGRenderer {
    fn clear(&mut self) {
        self.svg.set_inner_html("");
    }

    fn segment(&mut self, id: &str, segment: &Segment) {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("global document does not exists");

        if let None = document.get_element_by_id(id) {
            let svg_line = document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")
                .expect("can't create svg line element")
                .dyn_into::<SvgLineElement>()
                .expect("can't create svg line element");

            svg_line.set_id(id);
            svg_line.set_attribute("stroke", "black").expect("TODO: panic message");
            svg_line.set_attribute("stroke-dasharray", "4 1").expect("TODO: panic message");

            self.svg.append_child(&svg_line.dyn_into::<Node>().expect("")).expect("");
        }

        let svg_line = if let Some(element) = document.get_element_by_id(id) {
            element
                .dyn_into::<SvgLineElement>()
                .expect("can't create svg line element")
        } else {
            return;
        };

        svg_line.set_attribute("x1", &format!("{}", segment.start().x())).expect("TODO: panic message");
        svg_line.set_attribute("y1", &format!("{}", segment.start().y())).expect("TODO: panic message");

        svg_line.set_attribute("x2", &format!("{}", segment.end().x())).expect("TODO: panic message");
        svg_line.set_attribute("y2", &format!("{}", segment.end().y())).expect("TODO: panic message");
    }

    fn rectangle(&mut self, id: &str, rectangle: &Rectangle) {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("global document does not exists");

        if let None = document.get_element_by_id(id) {
            let svg_rectangle = document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")
                .expect("can't create svg rectangle element")
                .dyn_into::<SvgRectElement>()
                .expect("can't create svg rectangle element");

            svg_rectangle.set_id(id);

            self.svg.append_child(&svg_rectangle.dyn_into::<Node>().expect("")).expect("");            
        }

        let svg_rectangle = if let Some(element) = document.get_element_by_id(id) {
            element
                .dyn_into::<SvgRectElement>()
                .expect("can't create svg rectangle element")
        } else {
            return;
        };

        svg_rectangle.set_attribute("x", &format!("{}", rectangle.top_left().x())).expect("TODO: panic message");
        svg_rectangle.set_attribute("y", &format!("{}", rectangle.top_left().y())).expect("TODO: panic message");

        svg_rectangle.set_attribute("width", &format!("{}", rectangle.width())).expect("TODO: panic message");
        svg_rectangle.set_attribute("height", &format!("{}", rectangle.height())).expect("TODO: panic message");
    }

    fn circle(&mut self, id: &str, circle: &Circle) {
        todo!()
    }

    fn ellipse(&mut self, id: &str, ellipse: &Ellipse) {
        todo!()
    }

    fn path(&mut self, id: &str, path: &Path) {
        todo!()
    }
}
