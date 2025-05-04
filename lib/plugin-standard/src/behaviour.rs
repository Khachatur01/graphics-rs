use core::entity::behaviour::Behaviour;
use core::entity::Entity;
use geometry::figure::point::Point;

pub struct MoveDraw {
    pub mouse_down: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, start: &Point, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, start: &Point, current_point: &Point),
}
impl Behaviour for MoveDraw {}

pub struct ClickDraw {
    pub mouse_down: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, current_point: &Point),
}
impl Behaviour for ClickDraw {}

pub struct AddChild {
    pub add_child: fn(entity: &mut Entity, child: Entity),
}
impl Behaviour for AddChild {}
