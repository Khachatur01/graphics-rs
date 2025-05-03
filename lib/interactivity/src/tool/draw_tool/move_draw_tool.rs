use element_view::{Element, ElementView};
use geometry::figure::point::Point;

mod render;
mod tool;

pub struct MoveDrawTool<Id: 'static, El: 'static> {
    start: Option<Point>,
    // drawable: Option<Box<dyn ElementView<Id>>>,
    drawable: Option<Element<Id, El>>,
    // build_drawable: Box<dyn Fn() -> dyn ElementView<Id>>,
    // pub event_channel: EventChannel<Drawable>
}

impl<Id, El> MoveDrawTool<Id, El> {
    pub fn new() -> MoveDrawTool<Id, El> {
        Self {
            start: None,
            drawable: None,
            // build_drawable: Box::new(|| {
            // }),
            // event_channel: Default::default()
        }
    }

    pub fn end_drawing(&mut self) {
        let Some(drawable) = self.drawable.take() else {
            return;
        };

        self.start.take();

        // let _ = self.event_channel.end_drawing.send(EndDrawing { drawable: drawable.clone() });
    }
}
