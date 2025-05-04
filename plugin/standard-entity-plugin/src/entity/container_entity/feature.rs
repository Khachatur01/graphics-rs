use crate::entity::container_entity::ContainerEntity;
use crate::AddChild;
use core::entity::Entity;
use geometry::figure::rectangle::Rectangle;
use standard_rendering_plugin::Render;
use standard_rendering_plugin::renderer::Renderer;
use standard_tool_plugin::Select;
use standard_tool_plugin::tool::select_tool::Selection;

#[inline]
pub fn add_features(entity: &mut Entity) {
    entity.add_feature(
        Render {
            render: |entity: &Entity, renderer: &mut dyn Renderer| {
                let container: &ContainerEntity = entity.model_ref();

                /* query all render features and call render method */
                container.children.iter().for_each(|entity| {
                    let Some(render) = entity.query::<Render>() else {
                        return;
                    };

                    (render.render)(entity, renderer)
                });
            }
        }
    );

    entity.add_feature(
        Select {
            select: |entity: &Entity, selection: &Rectangle| {
                let container: &ContainerEntity = entity.model_ref();

                todo!()
            }
        }
    );

    entity.add_feature(
        AddChild {
            add_child: |entity: &mut Entity, child: Entity| {
                let container: &mut ContainerEntity = entity.model_ref_mut();

                container.children.push(child);
            }
        }
    );
}
