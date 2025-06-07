use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature_derive::Feature;
use geometry::figure::rectangle::Rectangle;
use geometry::point::point_2d::Point2D;

pub mod tool;
pub mod traits;

#[derive(Feature, Clone)]
pub struct MoveDraw<Id: EntityId> {
    pub pointer_down: fn(entity: &mut Entity<Id>, current_point: &Point2D),
    pub pointer_move: fn(entity: &mut Entity<Id>, start: &Point2D, current_point: &Point2D),
    pub pointer_up: fn(entity: &mut Entity<Id>, start: &Point2D, current_point: &Point2D),
    pub finish: fn(entity: &mut Entity<Id>),
}

#[derive(Feature, Clone)]
pub struct ClickDraw<Id: EntityId> {
    pub pointer_down: fn(entity: &mut Entity<Id>, current_point: &Point2D),
    pub pointer_move: fn(entity: &mut Entity<Id>, current_point: &Point2D),
    pub pointer_up: fn(entity: &mut Entity<Id>, current_point: &Point2D),
    pub finish: fn(entity: &mut Entity<Id>),
}

#[derive(Feature, Clone)]
pub struct Select<Id: EntityId> {
    pub select: fn(entity: &Entity<Id>, selection: &Rectangle) -> bool,
}
