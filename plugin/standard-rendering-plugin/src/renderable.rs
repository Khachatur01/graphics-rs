use crate::renderer::renderer::Renderer;

pub trait Renderable {
    fn render(&self, renderer: &mut dyn Renderer);
}
