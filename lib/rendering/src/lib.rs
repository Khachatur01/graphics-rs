use geometry::figure::circle::Circle;
use geometry::figure::ellipse::Ellipse;
use geometry::figure::path::Path;
use geometry::figure::rectangle::Rectangle;
use geometry::figure::segment::Segment;

pub trait Render {
    fn render(&self, renderer: &mut dyn Renderer);
}

pub trait Renderer {
    fn clear(&mut self);
    fn segment(&mut self, id: &str, segment: &Segment);
    fn rectangle(&mut self, id: &str, rectangle: &Rectangle);
    fn circle(&mut self, id: &str, circle: &Circle);
    fn ellipse(&mut self, id: &str, ellipse: &Ellipse);
    fn path(&mut self, id: &str, path: &Path);
}
