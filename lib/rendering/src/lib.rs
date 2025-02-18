use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::path::Path;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;

pub trait Render<RendererImpl: Renderer> {
    fn render(&mut self, renderer: &mut RendererImpl);
}

pub trait Renderer {
    fn segment(&mut self, segment: &Segment);
    fn rectangle(&mut self, rectangle: &Rectangle);
    fn circle(&mut self, circle: &Circle);
    fn ellipse(&mut self, ellipse: &Ellipse);
    fn path(&mut self, path: &Path);
}
