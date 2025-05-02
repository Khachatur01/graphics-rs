use geometry::figure::point::Point;
use std::any::Any;

pub struct MoveDraw {
    pub mouse_down: fn(element: &mut dyn Any, current_point: &Point),
    pub mouse_move: fn(element: &mut dyn Any, start: &Point, current_point: &Point),
    pub mouse_up: fn(element: &mut dyn Any, start: &Point, current_point: &Point),
}

pub struct ClickDraw {
    pub mouse_down: fn(element: &mut dyn Any, points: &Vec<Point>, current_point: &Point),
    pub mouse_move: fn(element: &mut dyn Any, points: &Vec<Point>, current_point: &Point),
    pub mouse_up: fn(element: &mut dyn Any, points: &Vec<Point>, current_point: &Point),
}
