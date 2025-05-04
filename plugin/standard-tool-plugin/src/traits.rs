use core::entity::Entity;

pub trait AddEntity {
    fn add_entity(&mut self, entity: Entity);
}

pub trait GetEntities {
    fn get_entities(&self) -> &Vec<Entity>;
}
