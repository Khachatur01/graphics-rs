use crate::tool::draw_tool::draw_mode::MoveDraw;
use crate::tool::draw_tool::move_draw_tool::MoveDrawTool;
use crate::tool::Tool;
use crate::Interactive;
use element_view::query;
use geometry::figure::point::Point;

impl<Id, El> Interactive for MoveDrawTool<Id, El> {
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

        let move_draws: Vec<&MoveDraw<Id, El>> = query(drawable);
        for move_draw in move_draws {
            (move_draw.mouse_move)(drawable, &start, point);
        }

        // let drawable = &mut **drawable;
        // let drawable: &dyn ElementView<Id> = *drawable;

        // let behaviours: Vec<MoveDraw> = query(drawable);
        // let drawable = drawable as &mut dyn Any;

        // let move_draw_components = components
        //     .iter_mut()
        //     .filter_map(|component| {
        //         component.downcast_mut::<MoveDraw>()
        //     })
        //     .collect::<Vec<&mut MoveDraw>>();
        //
        //
        // let drawable: &mut dyn Any = drawable;
        // for move_draw_component in behaviours {
        //     (move_draw_component.mouse_move)(drawable, &start, &point);
        // }

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

impl<Id: 'static, T: 'static> Tool for MoveDrawTool<Id, T> {}
