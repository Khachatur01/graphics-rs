use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Display;
use crate::{Feature, Model};

pub trait Id: Display {}

pub struct Entity {
    id: Box<dyn Id>,
    model: Box<dyn Any>,
    features: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: impl Id + 'static, model: impl Model + 'static) -> Self {
        Self {
            id: Box::new(id),
            model: Box::new(model),
            features: HashMap::new()
        }
    }

    pub fn id(&self) -> &dyn Id {
        self.id.as_ref()
    }

    pub fn add_feature<M: Feature + 'static>(&mut self, feature: M) {
        self.features.insert(TypeId::of::<M>(), Box::new(feature));
    }

    pub fn model_ref<M: Model + 'static>(&self) -> &M {
        self.model.downcast_ref().expect("Can't downcast model to specified type reference!")
    }

    pub fn model_ref_mut<M: Model + 'static>(&mut self) -> &mut M {
        self.model.downcast_mut().expect("Can't downcast model to specified type mutable reference!")
    }
}

impl Entity {
    pub fn query<B: Feature + 'static>(&self) -> Option<&B> {
        let feature_type_id: TypeId = TypeId::of::<B>();

        self
            .features
            .get(&feature_type_id)
            .and_then(|feature| feature.downcast_ref::<B>())
    }
}
