use crate::tool::draw_tool::draw_mode::MoveDraw;
use crate::tool::draw_tool::move_draw_tool::event_channel::{MouseDown, MouseMove, MouseUp};
use crate::tool::draw_tool::move_draw_tool::MoveDrawTool;
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;

impl<Drawable: MoveDraw> Interactive for MoveDrawTool<Drawable> {
    fn mouse_down(&mut self, point: &Point) {
        let mut drawable: Drawable = Drawable::default();

        self.start.replace(point.clone());
        drawable.mouse_down(point);

        let _ = self.event_channel.mouse_down.send(MouseDown { drawable: drawable.clone(), point: point.clone() });

        self.drawable = Some(drawable);
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(drawable) = &mut self.drawable else {
            return;
        };

        let Some(start) = self.start else { return; };
        drawable.mouse_move(&start, point);

        let _ = self.event_channel.mouse_move.send(MouseMove { drawable: drawable.clone(), point: point.clone() });
    }

    fn mouse_up(&mut self, point: &Point) {
        let Some(drawable) = &mut self.drawable else {
            return;
        };

        /* Take value from start point to make sure after mouse up action start point is None */
        let Some(start) = self.start.take() else { return; };
        drawable.mouse_up(&start, point);

        let _ = self.event_channel.mouse_up.send(MouseUp { drawable: drawable.clone(), point: point.clone() });

        self.end_drawing();
    }
}

impl<Drawable: MoveDraw> Tool for MoveDrawTool<Drawable> {}
