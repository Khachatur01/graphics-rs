use geometry::figure::point::Point;

pub mod tool;
mod channel;

pub trait Interactive {
    fn mouse_down(&mut self, point: &Point);

    fn mouse_move(&mut self, point: &Point);

    fn mouse_up(&mut self, point: &Point);
}
