use entity_model_feature::entity::Entity;
use crate::ToSVG;

pub trait SVGRenderer {
    fn remove(&mut self, id: &str);
    fn add(&mut self, entity: &Entity);
    fn modify(&mut self, entity: &Entity);
}
