mod canvas_renderer;
mod svg_renderer;

use crate::canvas_renderer::CanvasRenderer;
use crate::svg_renderer::SVGRenderer;
use geometry::figure::point::Point;
use interactivity::tool::draw_tool::draw_mode::MoveDraw;
use interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use interactivity::tool::Tool;
use rendering::{Render, Renderer};
use std::fmt::Display;
use std::rc::Rc;
use view_port::element_view::geometric::rectangle_view::RectangleElement;
use view_port::element_view::ElementView;
use view_port::identifier::Identifier;
use view_port::ViewPort;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ElementId {
    owner_id: String,
    index: usize,
}

impl Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}_{}", self.owner_id, self.index))
    }
}

impl Identifier for ElementId {
    fn generate() -> Self {
        ElementId {
            owner_id: "asdasd".to_string(),
            index: js_sys::Date::new_0().get_milliseconds() as usize,
        }
    }
}

#[wasm_bindgen]
pub struct Whiteboard {
    view_port: Rc<ViewPort<ElementId>>,
    active_tool: Box<dyn Tool>,
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn new() -> Self {
        let view_port = Rc::new(ViewPort::<ElementId>::new());

        let move_draw_tool = Self::create_move_draw_tool::<RectangleElement<ElementId>>(Rc::clone(&view_port));

        Self {
            view_port,
            active_tool: Box::new(move_draw_tool),
        }
    }

    pub fn mouse_down(&mut self, x: f64, y: f64) {
        self.active_tool.mouse_down(&Point::new(x, y));
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        self.active_tool.mouse_move(&Point::new(x, y));
    }

    pub fn mouse_up(&mut self, x: f64, y: f64) {
        self.active_tool.mouse_up(&Point::new(x, y));
    }

    pub fn render_canvas(&self, renderer: &mut CanvasRenderer) {
        renderer.clear();
        self.view_port.render(renderer);
        self.active_tool.render(renderer);
    }

    pub fn render_svg(&self, renderer: &mut SVGRenderer) {
        self.view_port.render(renderer);
        self.active_tool.render(renderer);
    }

    fn create_move_draw_tool<Element: ElementView<ElementId> + MoveDraw + 'static>(view_port: Rc<ViewPort<ElementId>>) -> MoveDrawTool<Element> {
        let mut move_draw_tool: MoveDrawTool<Element> = MoveDrawTool::new();

        move_draw_tool.event_channel.mouse_down.subscribe(move |mouse_down| {
            log(&format!("Mouse down: {}, {}", mouse_down.point.x(), mouse_down.point.y()));
        });

        move_draw_tool.event_channel.mouse_move.subscribe(move |mouse_move| {
            log(&format!("Mouse move: {}, {}", mouse_move.point.x(), mouse_move.point.y()));
        });

        move_draw_tool.event_channel.mouse_up.subscribe(move |mouse_up| {
            log(&format!("Mouse up: {}, {}", mouse_up.point.x(), mouse_up.point.y()));
        });

        move_draw_tool.event_channel.end_drawing.subscribe(move |end_drawing| {
            view_port.add_element(end_drawing.drawable).unwrap();

            let count = view_port.elements().read().unwrap().len();
            log(&format!("Elements count: {}", count));
        });

        move_draw_tool
    }
}
