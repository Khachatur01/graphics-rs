pub mod svg_element;
pub mod incremental_svg_renderer;
pub mod styles;

use entity_model_feature::entity::Entity;
use entity_model_feature_derive::Feature;
use crate::svg_element::{SVGElement, SVGElement1};

#[derive(Feature, Clone)]
pub struct ToSVG {
    pub to_svg: fn(entity: &Entity) -> SVGElement,
}

#[derive(Feature, Clone)]
pub struct UpdateSVG {
    pub update_svg: fn(entity: &Entity, svg_element1: &mut dyn SVGElement1),
}
