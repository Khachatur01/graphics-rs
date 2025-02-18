use geometry::figure::point::Point;

pub trait Draw {
    fn mouse_down(&mut self, points: &Vec<Point>, mouse_position: &Point);
    fn mouse_move(&mut self, points: &Vec<Point>, mouse_position: &Point);
    fn mouse_up(&mut self, points: &Vec<Point>, mouse_position: &Point);
}
