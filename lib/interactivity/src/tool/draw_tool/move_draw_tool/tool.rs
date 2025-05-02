use crate::tool::draw_tool::move_draw_tool::MoveDrawTool;
use crate::tool::Tool;
use crate::Interactive;
use geometry::figure::point::Point;

impl<Id> Interactive for MoveDrawTool<Id> {
    fn mouse_down(&mut self, point: &Point) {
        // let mut drawable: Drawable = (self.build_drawable)();
        // self.start.replace(point.clone());
        // drawable.mouse_down(point);
        //
        // self.drawable = Some(drawable);
    }

    fn mouse_move(&mut self, point: &Point) {
        let Some(drawable) = &mut self.drawable else {
            return;
        };

        let Some(start) = self.start else { return; };
        // drawable.mouse_move(&start, point);
    }

    fn mouse_up(&mut self, point: &Point) {
        // let Some(drawable) = &mut self.drawable else {
        //     return;
        // };
        // 
        // /* Take value from start point to make sure after mouse up action start point is None */
        // let Some(start) = self.start.take() else { return; };
        // drawable.mouse_up(&start, point);
        // 
        // self.end_drawing();
    }
}

impl<Id> Tool for MoveDrawTool<Id> {}
