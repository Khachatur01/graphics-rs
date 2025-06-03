use crate::entity::Entity;
use crate::entity_id::EntityId;

pub struct Container<Id: EntityId> {
    id: Id,
    entities: Vec<Entity<Id>>,
}

impl<Id: EntityId> Container<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            entities: vec![],
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn add_entity(&mut self, entity: Entity<Id>) {
        self.entities.push(entity);
    }

    pub fn entities(&self) -> &Vec<Entity<Id>> {
        &self.entities
    }
}
