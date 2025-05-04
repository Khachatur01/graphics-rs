use crate::entity::{Entity, Id};

pub struct Container {
    id: Box<dyn Id>,
    entities: Vec<Entity>,
}

impl Container {
    pub fn new(id: impl Id + 'static) -> Self {
        Self {
            id: Box::new(id),
            entities: vec![],
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
