use std::fmt::Display;
use crate::{Feature, Model};
use crate::feature_set::FeatureSet;

pub trait Id: Display {}

pub struct Entity {
    id: Box<dyn Id>,
    model: Box<dyn Model>,
    feature_set: FeatureSet,
}

impl Entity {
    pub fn new<const N: usize>(id: impl Id + 'static,
                               model: impl Model + 'static,
                               feature_set: FeatureSet) -> Self {

        Self {
            id: Box::new(id),
            model: Box::new(model),
            feature_set,
        }
    }

    pub fn id(&self) -> &dyn Id {
        self.id.as_ref()
    }

    pub fn add_feature<M: Feature + 'static>(&mut self, feature: M) {
        self.feature_set.add_feature(feature);
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
        self.feature_set.query::<B>()
    }
}
