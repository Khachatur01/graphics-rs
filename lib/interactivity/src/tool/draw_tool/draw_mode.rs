use element_view::Element;
use geometry::figure::point::Point;
use std::any::Any;

#[derive(Clone)]
pub struct MoveDraw<Id, El: 'static> {
    pub mouse_down: fn(element: &mut Element<Id, El>, current_point: &Point),
    pub mouse_move: fn(element: &mut Element<Id, El>, start: &Point, current_point: &Point),
    pub mouse_up: fn(element: &mut Element<Id, El>, start: &Point, current_point: &Point),
}

#[derive(Clone)]
pub struct ClickDraw {
    pub mouse_down: fn(element: &mut dyn Any, points: &Vec<Point>, current_point: &Point),
    pub mouse_move: fn(element: &mut dyn Any, points: &Vec<Point>, current_point: &Point),
    pub mouse_up: fn(element: &mut dyn Any, points: &Vec<Point>, current_point: &Point),
}
