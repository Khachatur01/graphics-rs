use crate::tool::select_tool::SelectTool;
use geometry::figure::point::Point;
use geometry::figure::segment::Segment;
use rendering::{Render, Renderer};

impl Render for SelectTool {
    fn render(&self, renderer: &mut dyn Renderer) {
        let Some(selection) = self.selection else {
            return;
        };

        let x = selection.top_left().x();
        let y = selection.top_left().y();
        let width = selection.width();
        let height = selection.height();

        renderer.segment(&Segment::new(Point::new(x, y), Point::new(x + width, y)));
        renderer.segment(&Segment::new(Point::new(x + width, y), Point::new(x + width, y + height)));
        renderer.segment(&Segment::new(Point::new(x + width, y + height), Point::new(x, y + height)));
        renderer.segment(&Segment::new(Point::new(x, y + height), Point::new(x, y)));
    }
}
