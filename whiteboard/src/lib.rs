mod canvas_renderer;

use crate::canvas_renderer::CanvasRenderer;
use geometry::figure::point::Point;
use interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use interactivity::tool::Tool;
use rendering::{Render, Renderer};
use std::rc::Rc;
use view_port::element_view::geometric::rectangle_view::RectangleElement;
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

#[wasm_bindgen]
pub struct Whiteboard {
    view_port: Rc<ViewPort<ElementId>>,
    active_tool: Box<dyn Tool>,
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn new() -> Self {
        let view_port = Rc::new(ViewPort::<ElementId>::new());

        let id: ElementId = ElementId {
            owner_id: String::from("asd"),
            index: 0,
        };
        let mut move_draw_tool = MoveDrawTool::new(RectangleElement::zero_sized(id));

        move_draw_tool.event_channel.mouse_down.subscribe(move |mouse_down| {
            log(&format!("Mouse down: {}, {}", mouse_down.point.x(), mouse_down.point.y()));
        });

        move_draw_tool.event_channel.mouse_move.subscribe(move |mouse_move| {
            log(&format!("Mouse move: {}, {}", mouse_move.point.x(), mouse_move.point.y()));
        });

        move_draw_tool.event_channel.mouse_up.subscribe(move |mouse_up| {
            log(&format!("Mouse up: {}, {}", mouse_up.point.x(), mouse_up.point.y()));
        });

        let view_port_clone = Rc::clone(&view_port);
        move_draw_tool.event_channel.end_drawing.subscribe(move |end_drawing| {
            log(&format!("Drawable: {}, {}, {}, {}", &end_drawing.drawable.rectangle().top_left().x(), &end_drawing.drawable.rectangle().top_left().y(), &end_drawing.drawable.rectangle().width(), &end_drawing.drawable.rectangle().height()));
            view_port_clone.add_element(end_drawing.drawable).unwrap();

            let count = view_port_clone.elements().read().unwrap().len();
            log(&format!("Elements count: {}", count));
        });

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

    pub fn render(&self, renderer: &mut CanvasRenderer) {
        renderer.clear();
        self.view_port.render(renderer);
        self.active_tool.render(renderer);
    }
}
