mod element_id;
mod renderer;

use crate::element_id::ElementId;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;
use plugin_rendering::style::shape_style::ShapeStyle;
use plugin_rendering::{Renderable, Renderer};
use plugin_standard::entity::geometric::rectangle_entity::RectangleEntity;
use plugin_standard::interactivity::tool::draw_tool::move_draw_tool::MoveDrawTool;
use plugin_standard::interactivity::tool::select_tool::SelectTool;
use plugin_standard::interactivity::tool::Tool;
use plugin_standard::view_port::ViewPort;
use renderer::canvas_renderer::CanvasRenderer;
use renderer::svg_renderer::SVGRenderer;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}



#[wasm_bindgen]
pub struct Whiteboard {
    id: ElementId,
    view_port: ViewPort,
    active_tool: Box<dyn Tool>,
}

#[wasm_bindgen]
impl Whiteboard {
    pub fn new() -> Self {
        let owner_id = "asdasd";

        Self {
            id: ElementId::with_owner_id(owner_id),
            view_port: ViewPort::new(ElementId::with_owner_id(owner_id)),
            active_tool: Box::new(SelectTool::new()),
        }
    }

    pub fn activate_rectangle_tool(&mut self) {
        let draw_rectangle_tool = MoveDrawTool::new(
            self.view_port.clone(),
            move || {
                let id: ElementId = ElementId::with_owner_id("weuif");
                RectangleEntity::new(id, Rectangle::zero_sized(Point::default()), ShapeStyle::default())
            }
        );

        self.activate_tool(draw_rectangle_tool);
    }

    pub fn activate_select_tool(&mut self) {
        self.activate_tool(SelectTool::new());
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

    fn activate_tool(&mut self, tool: impl Tool + 'static) {
        self.active_tool = Box::new(tool);
    }
}
