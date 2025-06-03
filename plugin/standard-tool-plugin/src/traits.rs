use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;

pub trait AddEntity<Id: EntityId> {
    fn add_entity(&mut self, entity: Entity<Id>);
}
