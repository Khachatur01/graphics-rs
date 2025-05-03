use core::entity::behaviour::Behaviour;
use core::entity::Entity;
use geometry::figure::point::Point;

pub mod click_draw_tool;
pub mod move_draw_tool;

#[derive(Clone)]
pub struct MoveDraw<Id> {
    pub mouse_down: fn(entity: &mut Entity<Id>, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity<Id>, start: &Point, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity<Id>, start: &Point, current_point: &Point),
}
impl<Id> Behaviour for MoveDraw<Id> {}

#[derive(Clone)]
pub struct ClickDraw<Id> {
    pub mouse_down: fn(entity: &mut Entity<Id>, points: &Vec<Point>, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity<Id>, points: &Vec<Point>, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity<Id>, points: &Vec<Point>, current_point: &Point),
}
impl<Id> Behaviour for ClickDraw<Id> {}
