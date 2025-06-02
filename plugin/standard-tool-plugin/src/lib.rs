use entity_model_feature::entity::Entity;
use entity_model_feature_derive::Feature;
use geometry::figure::point::Point;
use geometry::figure::rectangle::Rectangle;

pub mod tool;
pub mod traits;

#[derive(Feature, Clone)]
pub struct MoveDraw {
    pub mouse_down: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, start: &Point, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, start: &Point, current_point: &Point),
    pub finish: fn(entity: &mut Entity),
}

#[derive(Feature, Clone)]
pub struct ClickDraw {
    pub mouse_down: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_move: fn(entity: &mut Entity, current_point: &Point),
    pub mouse_up: fn(entity: &mut Entity, current_point: &Point),
    pub finish: fn(entity: &mut Entity),
}

#[derive(Feature, Clone)]
pub struct Select {
    pub select: fn(entity: &Entity, selection: &Rectangle) -> bool,
}
