use geometry::figure::point::Point;

pub trait MoveDraw {
    fn mouse_down(&mut self, current_point: &Point);
    fn mouse_move(&mut self, start: &Point, current_point: &Point);
    fn mouse_up(&mut self, start: &Point, current_point: &Point);
}

pub trait ClickDraw {
    fn mouse_down(&mut self, points: &Vec<Point>, current_point: &Point);
    fn mouse_move(&mut self, points: &Vec<Point>, current_point: &Point);
    fn mouse_up(&mut self, points: &Vec<Point>, current_point: &Point);
}

pub enum DrawMode {
    Move {
        start: Option<Point>,
        drawable: Box<dyn MoveDraw>,
    },
    Click {
        points: Vec<Point>,
        drawable: Box<dyn ClickDraw>,
    },
}
