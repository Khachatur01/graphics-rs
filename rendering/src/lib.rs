mod canvas_renderer;

use view_port::view_port::ViewPort;

pub trait Renderer {
    fn render(&mut self, view_port: &ViewPort);
}
