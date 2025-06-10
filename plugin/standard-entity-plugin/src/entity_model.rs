use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::feature_set::FeatureSet;

pub trait DefaultEntity<Id: EntityId> {
    fn default_entity(id: Id) -> Entity<Id>;
}

pub trait StandardFeatureSet<Id: EntityId> {
    fn standard_feature_set() -> FeatureSet;
}
