use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;

pub trait RendererIncremental<Id: EntityId> {
    fn add(&mut self, entity: &Entity<Id>);
    fn modify(&mut self, entity: &Entity<Id>);
    fn remove(&mut self, id: &Id);
}
