pub trait Render<RendererImpl: Renderer> {
    fn render(&mut self, renderer: &mut RendererImpl);
}

pub trait Renderer {
    fn segment(&mut self, x: f64, y: f64, x1: f64, y1: f64);
    fn rect(&mut self, x: f64, y: f64, width: f64, height: f64);
}
