use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature_derive::Feature;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;

pub mod tool;
pub mod traits;

#[derive(Feature, Clone)]
pub struct MoveDraw<Id: EntityId> {
    pub mouse_down: fn(entity: &mut Entity<Id>, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity<Id>, start: &Point, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity<Id>, start: &Point, current_point: &Point),
    pub finish: fn(entity: &mut Entity<Id>),
}

#[derive(Feature, Clone)]
pub struct ClickDraw<Id: EntityId> {
    pub mouse_down: fn(entity: &mut Entity<Id>, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity<Id>, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity<Id>, current_point: &Point),
    pub finish: fn(entity: &mut Entity<Id>),
}

#[derive(Feature, Clone)]
pub struct Select<Id: EntityId> {
    pub select: fn(entity: &Entity<Id>, selection: &Rectangle) -> bool,
}
