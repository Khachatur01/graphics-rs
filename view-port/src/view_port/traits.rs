use geometry::shape::point::Point;

pub trait MouseEvents {
    fn mouse_down(&mut self, point: &Point);

    fn mouse_move(&mut self, point: &Point);

    fn mouse_up(&mut self, point: &Point);
}
