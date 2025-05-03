pub mod render {
    use crate::entity::container_entity::ContainerEntity;
    use crate::entity::container_entity::Entity;
    use plugin_rendering::behaviour::Render;
    use plugin_rendering::Renderer;

    pub fn render(entity: &Entity, renderer: &mut dyn Renderer) {
        let container: &ContainerEntity = entity.model_ref();

        /* query all render behaviours and call render method */
        container.children.iter().for_each(|entity| {
            let Some(render) = entity.query::<Render>() else {
                return;
            };

            (render.render)(entity, renderer)
        });
    }
}

pub mod add_child {
    use crate::entity::container_entity::{ContainerEntity, Entity};

    pub fn add_child(entity: &mut Entity, child: Entity) {
        let container: &mut ContainerEntity = entity.model_ref_mut();

        container.children.push(child);
    }
}
