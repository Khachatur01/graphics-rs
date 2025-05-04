use core::entity::Entity;
use core_derive::Feature;
use geometry::figure::point::Point;

pub mod tool;

#[derive(Feature)]
pub struct MoveDraw {
    pub mouse_down: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, start: &Point, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, start: &Point, current_point: &Point),
}

#[derive(Feature)]
pub struct ClickDraw {
    pub mouse_down: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, current_point: &Point),
}
