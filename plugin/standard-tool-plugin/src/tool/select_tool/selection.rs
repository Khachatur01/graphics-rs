use algebra::linear::transformation::Transformation;
use entity_model_feature::entity_id::EntityId;

pub struct Selection<Id> {
    entities: Vec<Id>,
    transformations: Vec<Transformation>,
}

impl<Id: EntityId> Selection<Id> {
    pub fn new(entities: &[Id]) -> Self {
        Self {
            entities: entities.to_vec(),
            transformations: vec![],
        }
    }

    pub fn add_transformation(&mut self, transformation: Transformation) {
        self.transformations.push(transformation);
    }
}
