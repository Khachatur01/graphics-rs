use crate::AddChild;
use core::entity::Entity;
use core::feature_set::FeatureSet;
use core::EntityId;
use core::Feature;
use core_derive::Model;
use geometry::figure::rectangle::Rectangle;
use serde::Serialize;
use standard_rendering_plugin::renderer::Renderer;
use standard_rendering_plugin::Render;
use standard_tool_plugin::Select;

#[derive(Model, Serialize)]
pub struct ContainerModel {
    pub children: Vec<Entity>,
}

impl ContainerModel {
    pub fn entity(id: impl EntityId + 'static) -> Entity {
        Entity::new(
            id,
            ContainerModel { children: vec![] },
            Self::standard_feature_set(),
        )
    }

    pub fn standard_feature_set() -> FeatureSet {
        FeatureSet::from([
            Self::render().boxed(),
            Self::select().boxed(),
            Self::add_child().boxed(),
        ])
    }

    pub fn render() -> Render {
        Render {
            render: |entity: &Entity, renderer: &mut dyn Renderer| {
                let container: &Self = entity.model_ref();

                /* query all render features and call render method */
                for child in container.children.iter() {
                    let Some(render) = child.query::<Render>() else {
                        return;
                    };

                    (render.render)(child, renderer)
                }
            },
        }
    }

    pub fn select() -> Select {
        Select {
            select: |entity: &Entity, selection: &Rectangle| {
                let container: &Self = entity.model_ref();

                todo!()
            },
        }
    }

    pub fn add_child() -> AddChild {
        AddChild {
            add_child: |entity: &mut Entity, child: Entity| {
                let container: &mut Self = entity.model_ref_mut();

                container.children.push(child);
            },
        }
    }
}
