pub mod svg_element;
pub mod property_map;

use crate::svg_element::SVGElement;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature_derive::Feature;

#[derive(Feature, Clone)]
pub struct ToSVG<Id: EntityId> {
    pub to_svg: fn(entity: &Entity<Id>) -> SVGElement,
}
