use entity_model_feature::entity::Entity;

pub trait AddEntity {
    fn add_entity(&mut self, entity: Entity);
}
