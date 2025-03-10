use geometry::figure::point::Point;
use rendering::Render;

pub trait MoveDraw: Render {
    fn mouse_down(&mut self, current_point: &Point);
    fn mouse_move(&mut self, start: &Point, current_point: &Point);
    fn mouse_up(&mut self, start: &Point, current_point: &Point);
}

pub trait ClickDraw: Render {
    fn mouse_down(&mut self, points: &Vec<Point>, current_point: &Point);
    fn mouse_move(&mut self, points: &Vec<Point>, current_point: &Point);
    fn mouse_up(&mut self, points: &Vec<Point>, current_point: &Point);
}
