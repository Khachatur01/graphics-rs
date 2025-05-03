use core::entity::behaviour::Behaviour;
use core::entity::Entity;
use geometry::figure::point::Point;

pub mod click_draw_tool;
pub mod move_draw_tool;

#[derive(Clone)]
pub struct MoveDraw {
    pub mouse_down: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, start: &Point, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, start: &Point, current_point: &Point),
}
impl Behaviour for MoveDraw {}

#[derive(Clone)]
pub struct ClickDraw {
    pub mouse_down: fn(entity: &mut Entity, points: &Vec<Point>, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, points: &Vec<Point>, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, points: &Vec<Point>, current_point: &Point),
}
impl Behaviour for ClickDraw {}
