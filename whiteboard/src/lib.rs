mod canvas_renderer;
mod svg_renderer;
mod element_id;

use crate::canvas_renderer::CanvasRenderer;
use crate::element_id::ElementId;
use crate::svg_renderer::SVGRenderer;
use geometry::figure::point::Point;
use interactivity::tool::draw_tool::draw_mode::MoveDraw;
use interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use interactivity::tool::select_tool::SelectTool;
use interactivity::tool::Tool;
use rendering::{Render, Renderer};
use std::rc::Rc;
use view_port::element_view::container_view::ContainerView;
use view_port::element_view::geometric::rectangle_view::RectangleElement;
use view_port::element_view::{DefaultWithId, ElementView};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


type SharedViewPort = Rc<ContainerView<ElementId>>;

#[wasm_bindgen]
pub struct Whiteboard {
    view_port: SharedViewPort,
    active_tool: Box<dyn Tool>,
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn new() -> Self {
        let owner_id = "asdasd";
        let view_port = Rc::new(ContainerView::<ElementId>::new(ElementId::with_owner_id(owner_id)));

        let select_tool: SelectTool<ElementId> = Self::create_select_tool();

        Self {
            view_port,
            active_tool: Box::new(select_tool),
        }
    }

    pub fn activate_rectangle_tool(&mut self) {
        let view_port = Rc::clone(&self.view_port);

        let draw_rectangle_tool: MoveDrawTool<RectangleElement<ElementId>> = Self::create_move_draw_tool(view_port);

        self.activate_tool(draw_rectangle_tool);
    }

    pub fn activate_select_tool(&mut self) {
        let select_tool: SelectTool<ElementId> = Self::create_select_tool();

        self.activate_tool(select_tool);
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
        renderer.clear();
        self.view_port.render(renderer);
        self.active_tool.render(renderer);
    }

    fn create_move_draw_tool<Element>(container: Rc<ContainerView<ElementId>>) -> MoveDrawTool<Element>
    where Element: ElementView<ElementId> + MoveDraw + DefaultWithId<ElementId> + 'static {

        let owner_id: String = container.id().owner_id().to_string();
        let mut move_draw_tool: MoveDrawTool<Element> = MoveDrawTool::new(move || {
            Element::default_with_id(ElementId::with_owner_id(owner_id.as_str()))
        });

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
            container.add_element(end_drawing.drawable).unwrap();

            let count = container.elements().read().unwrap().len();
            log(&format!("Elements count: {}", count));
        });

        move_draw_tool
    }

    fn create_select_tool() -> SelectTool<ElementId> {
        let select_tool: SelectTool<ElementId> = SelectTool::new();

        select_tool
    }

    fn activate_tool(&mut self, tool: impl Tool + 'static) {
        self.active_tool = Box::new(tool);
    }
}
