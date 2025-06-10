use crate::entity_model::{DefaultEntity, StandardFeatureSet};
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature::Feature;
use entity_model_feature_derive::Model;
use geometry::figure::path::command::line_to::LineTo;
use geometry::figure::path::command::move_to::MoveTo;
use geometry::figure::path::command::positioning::Positioning;
use geometry::figure::path::command::Command;
use geometry::figure::path::Path;
use getter_methods::GetterMethods;
use serde::{Deserialize, Serialize};
use standard_rendering_plugin::style::shape_style::ShapeStyle;
use standard_rendering_plugin::Render;
use standard_tool_plugin::MoveDraw;

#[derive(Model, Serialize, Deserialize, Clone, GetterMethods)]
pub struct PathModel {
    path: Path,
    style: ShapeStyle,
}

impl<Id: EntityId> DefaultEntity<Id> for PathModel {
    fn default_entity(id: Id) -> Entity<Id> {
        Entity::new(
            id,
            PathModel {
                path: Path::new(&[]),
                style: ShapeStyle::default(),
            },
            FeatureSet::empty(),
        )
    }
}

impl<Id: EntityId> StandardFeatureSet<Id> for PathModel {
    fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([
            Self::feature_move_draw::<Id>().boxed(),
            Self::feature_render::<Id>().boxed()
        ])
    }
}

impl PathModel {
    pub fn feature_move_draw<Id: EntityId>() -> MoveDraw<Id> {
        MoveDraw {
            pointer_down: |entity, current_point| {
                let free_hand: &mut PathModel = entity.model_ref_mut();

                free_hand.path.push_command(
                    Command::MoveTo(
                        MoveTo {
                            positioning: Positioning::Absolute,
                            to_point: current_point.clone(),
                        }
                    )
                );
            },
            pointer_move: |entity, start, current_point| {
                let free_hand: &mut PathModel = entity.model_ref_mut();

                free_hand.path.push_command(
                    Command::LineTo(
                        LineTo {
                            positioning: Positioning::Absolute,
                            to_point: current_point.clone(),
                        }
                    )
                );
            },
            pointer_up: |entity, start, current_point| {
                let free_hand: &mut PathModel = entity.model_ref_mut();
            },
            finish: |entity| {},
        }
    }

    pub fn feature_render<Id: EntityId>() -> Render<Id> {
        Render {
            render: |entity, renderer| {
                let free_hand: &PathModel = entity.model_ref();

                renderer.path(&free_hand.path, &free_hand.style);
            },
        }
    }
}
