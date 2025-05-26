use crate::entity::{Entity, Identifier};

pub struct Container<Id> {
    id: Id,
    entities: Vec<Entity>,
}

impl<Id: Identifier> Container<Id> {
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
