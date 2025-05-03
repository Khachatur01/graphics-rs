use behaviour::Behaviour;
pub use model::Model;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Display;

pub mod behaviour;
pub mod model;

pub trait Id: Display {}

pub struct Entity {
    id: Box<dyn Id>,
    model: Box<dyn Any>,
    behaviours: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: impl Id + 'static, model: impl Model + 'static) -> Self {
        Self {
            id: Box::new(id),
            model: Box::new(model),
            behaviours: HashMap::new()
        }
    }

    pub fn id(&self) -> &dyn Id {
        self.id.as_ref()
    }

    pub fn add_behaviour<M: Behaviour + 'static>(&mut self, behaviour: M) {
        self.behaviours.insert(TypeId::of::<M>(), Box::new(behaviour));
    }

    pub fn model_ref<M: Model + 'static>(&self) -> &M {
        self.model.downcast_ref().expect("Can't downcast model to specified type reference!")
    }

    pub fn model_ref_mut<M: Model + 'static>(&mut self) -> &mut M {
        self.model.downcast_mut().expect("Can't downcast model to specified type mutable reference!")
    }
}

impl Entity {
    pub fn query<B: Behaviour + 'static>(&self) -> Option<&B> {
        let behaviour_type_id: TypeId = TypeId::of::<B>();

        self
            .behaviours
            .get(&behaviour_type_id)
            .and_then(|behaviour| behaviour.downcast_ref::<B>())
    }
}
