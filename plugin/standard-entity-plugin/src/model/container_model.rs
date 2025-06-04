use crate::AddChild;
use entity_model_feature::entity::Entity;
use entity_model_feature::entity_id::EntityId;
use entity_model_feature::feature_set::FeatureSet;
use entity_model_feature::Feature;
use entity_model_feature_derive::Model;
use geometry::figure::rectangle::Rectangle;
use serde::Serialize;
use standard_rendering_plugin::renderer::Renderer;
use standard_rendering_plugin::Render;
use standard_tool_plugin::Select;

#[derive(Model, Clone, Serialize)]
pub struct ContainerModel<Id: EntityId> {
    pub children: Vec<Entity<Id>>,
}

impl<Id: EntityId> ContainerModel<Id> {
    pub fn entity(id: Id) -> Entity<Id> {
        Entity::new(
            id,
            ContainerModel::<Id> { children: vec![] },
            Self::standard_feature_set(),
        )
    }

    pub fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([
            Self::feature_render().boxed(),
            Self::feature_select().boxed(),
            Self::feature_add_child().boxed(),
        ])
    }

    pub fn feature_render() -> Render<Id> {
        Render {
            render: |entity, renderer: &mut dyn Renderer| {
                let container: &Self = entity.model_ref();

                /* query all render features and call render method */
                for child in container.children.iter() {
                    let Some(render) = child.query::<Render<Id>>() else {
                        return;
                    };

                    (render.render)(child, renderer)
                }
            },
        }
    }

    pub fn feature_select() -> Select<Id> {
        Select {
            select: |entity, selection: &Rectangle| {
                let container: &Self = entity.model_ref();

                todo!()
            },
        }
    }

    pub fn feature_add_child() -> AddChild<Id> {
        AddChild {
            add_child: |entity, child| {
                let container: &mut Self = entity.model_ref_mut();

                container.children.push(child);
            },
        }
    }
}
