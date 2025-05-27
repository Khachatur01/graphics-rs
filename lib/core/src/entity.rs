use crate::feature_set::FeatureSet;
use crate::{AsSerialize, Feature, Model};
use dyn_clone::DynClone;
use std::fmt::Display;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub trait Identifier: Display + DynClone + AsSerialize {}


pub struct Entity {
    id: Box<dyn Identifier>,
    model: Box<dyn Model>,
    feature_set: FeatureSet,
}

impl Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Entity", 2)?;
        state.serialize_field("id", &self.id.as_serialize())?;
        state.serialize_field("model", &self.model.as_serialize())?;
        state.end()
    }
}

impl Entity {
    pub fn new(
        id: impl Identifier + 'static,
        model: impl Model + 'static,
        feature_set: FeatureSet,
    ) -> Self {
        Self {
            id: Box::new(id),
            model: Box::new(model),
            feature_set,
        }
    }

    pub fn id(&self) -> &dyn Identifier {
        self.id.as_ref()
    }

    pub fn add_feature<M: Feature + 'static>(&mut self, feature: M) {
        self.feature_set.add_feature(feature);
    }

    pub fn model_ref<M: Model + 'static>(&self) -> &M {
        self.model
            .as_any()
            .downcast_ref()
            .expect("Can't downcast model to specified type reference!")
    }

    pub fn model_ref_mut<M: Model + 'static>(&mut self) -> &mut M {
        self.model
            .as_any_mut()
            .downcast_mut()
            .expect("Can't downcast model to specified type mutable reference!")
    }
}

impl Entity {
    pub fn query<B: Feature + 'static>(&self) -> Option<&B> {
        self.feature_set.query::<B>()
    }
}
