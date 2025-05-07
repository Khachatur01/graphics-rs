use geometry::figure::point::Point;
use standard_rendering_plugin::renderer::Renderable;

pub mod select_tool;
pub mod draw_tool;

pub trait Tool: Renderable {
    fn mouse_down(&mut self, point: &Point);

    fn mouse_move(&mut self, point: &Point);

    fn mouse_up(&mut self, point: &Point);
}
