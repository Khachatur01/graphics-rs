use geometry::shape::point::Point;

pub trait MouseEvents {
    fn make_mouse_down(&mut self, point: &Point);

    fn make_mouse_move(&mut self, point: &Point);

    fn make_mouse_up(&mut self, point: &Point);
}
