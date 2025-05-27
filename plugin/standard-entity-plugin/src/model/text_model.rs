use crate::Entity;
use core::feature_set::FeatureSet;
use core::EntityId;
use core_derive::Model;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use standard_rendering_plugin::style::text_style::TextStyle;

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct TextModel {
    text: String,
    style: TextStyle,
}

impl TextModel {
    pub fn entity(id: impl EntityId + 'static, text: &str, style: TextStyle) -> Entity {
        Entity::new(
            id,
            TextModel { text: text.to_string(), style },
            FeatureSet::empty(),
        )
    }
}
