use crate::entity::Entity;
use crate::EntityId;

pub struct Container<Id> {
    id: Id,
    entities: Vec<Entity>,
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

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
