use crate::entity_id::EntityId;
use crate::feature_set::FeatureSet;
use crate::{Feature, Model};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Clone)]
pub struct Entity<Id: EntityId> {
    id: Id,
    model: Box<dyn Model>,
    feature_set: FeatureSet,
}

impl<Id: EntityId> Serialize for Entity<Id> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Entity", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("model", &self.model.as_serialize())?;
        state.end()
    }
}

impl<Id: EntityId> Entity<Id> {
    pub fn new(
        id: Id,
        model: impl Model + 'static,
        feature_set: FeatureSet,
    ) -> Self {
        Self {
            id,
            model: Box::new(model),
            feature_set,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
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

impl<Id: EntityId> Entity<Id> {
    pub fn query<B: Feature + 'static>(&self) -> Option<&B> {
        self.feature_set.query::<B>()
    }
}
