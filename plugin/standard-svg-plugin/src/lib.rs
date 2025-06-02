mod svg_element;

use entity_model_feature::entity::Entity;
use entity_model_feature_derive::Feature;
use crate::svg_element::SVGElement;

#[derive(Feature, Clone)]
pub struct ToSVG {
    to_svg: fn(entity: &Entity) -> SVGElement,
}

#[derive(Feature, Clone)]
pub struct FromSVG {
    from_svg: fn(entity: &mut Entity, svg_element: &SVGElement),
}
