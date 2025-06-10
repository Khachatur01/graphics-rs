use crate::entity_model::{DefaultEntity, StandardFeatureSet};
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature_derive::Model;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use standard_rendering_plugin::style::text_style::TextStyle;

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct TextModel {
    text: String,
    style: TextStyle,
}

impl<Id: EntityId> DefaultEntity<Id> for TextModel {
    fn default_entity(id: Id) -> Entity<Id> {
        Entity::new(
            id,
            TextModel {
                text: String::new(),
                style: TextStyle::default(),
            },
            FeatureSet::empty(),
        )
    }
}

impl<Id: EntityId> StandardFeatureSet<Id> for TextModel {
    fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([])
    }
}

impl TextModel {
    pub fn entity<Id: EntityId>(id: Id, text: &str, style: TextStyle) -> Entity<Id> {
        Entity::new(
            id,
            TextModel { text: text.to_string(), style },
            FeatureSet::empty(),
        )
    }
}
