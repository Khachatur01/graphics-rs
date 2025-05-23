use core::entity::Id;
use core::entity::{Entity};
use core_derive::Model;
use geometry::figure::rectangle::Rectangle;
use standard_rendering_plugin::Render;
use standard_rendering_plugin::renderer::Renderer;
use standard_tool_plugin::Select;
use crate::AddChild;
use core::Feature;
use core::feature_set::FeatureSet;

#[derive(Model)]
pub struct ContainerEntity {
    pub children: Vec<Entity>,
}

impl ContainerEntity {
    pub fn with_standard_feature_set(id: impl Id + 'static) -> Entity {
        Entity::new(
            id,
            ContainerEntity {
                children: vec![]
            },
            Self::standard_feature_set()
        )
    }

    pub fn standard_feature_set() -> FeatureSet {
        FeatureSet::from(
            [
                Self::render().boxed(),
                Self::select().boxed(),
                Self::add_child().boxed(),
            ]
        )
    }

    pub fn render() -> Render {
        Render {
            render: |entity: &Entity, renderer: &mut dyn Renderer| {
                let container: &ContainerEntity = entity.model_ref();

                /* query all render features and call render method */
                for child in container.children.iter() {
                    let Some(render) = child.query::<Render>() else {
                        return;
                    };

                    (render.render)(child, renderer)
                }
            }
        }
    }

    pub fn select() -> Select {
        Select {
            select: |entity: &Entity, selection: &Rectangle| {
                let container: &ContainerEntity = entity.model_ref();

                todo!()
            }
        }
    }

    pub fn add_child() -> AddChild {
        AddChild {
            add_child: |entity: &mut Entity, child: Entity| {
                let container: &mut ContainerEntity = entity.model_ref_mut();

                container.children.push(child);
            }
        }
    }
}
