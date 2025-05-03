use crate::entity::container_entity::ContainerEntity;
use core::entity::Entity;
use rendering::behaviour::Render;
use rendering::Renderer;

pub fn render(entity: &Entity, renderer: &mut dyn Renderer) {
    let container: &ContainerEntity = entity.model_ref();

    /* query all render behaviours and call render method */
    container.entities.iter().for_each(|entity| {
        let Some(render) = entity.query::<Render>() else {
            return;
        };

        (render.render)(entity, renderer)
    });
}
