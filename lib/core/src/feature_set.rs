use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::Feature;

pub struct FeatureSet {
    features: HashMap<TypeId, Box<dyn Feature>>,
}

impl FeatureSet {
    pub fn empty() -> Self {
        Self {
            features: HashMap::new(),
        }
    }

    pub fn add_feature<M: Feature + 'static>(&mut self, feature: M) {
        self.features.insert(TypeId::of::<M>(), Box::new(feature));
    }

    pub fn extend(&mut self, extension: FeatureSet) -> &Self {
        self.features.extend(extension.features);

        self
    }

    pub fn query<B: Feature + 'static>(&self) -> Option<&B> {
        let feature_type_id: TypeId = TypeId::of::<B>();

        self
            .features
            .get(&feature_type_id)
            .and_then(|feature| feature.downcast_ref::<B>())
    }
}

impl<const N: usize> From<[Box<dyn Feature>; N]> for FeatureSet {
    fn from(features: [Box<dyn Feature>; N]) -> Self {
        Self {
            features: features
                .into_iter()
                .map(|feature| (feature.as_ref().type_id(), feature))
                .collect()
        }
    }
}
