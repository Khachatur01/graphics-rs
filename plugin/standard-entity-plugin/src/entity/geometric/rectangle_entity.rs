mod feature;

use crate::entity::geometric::rectangle_entity::feature::add_features;
use core::entity::Entity;
use core::entity::Id;
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use core_derive::Model;
use standard_rendering_plugin::style::shape_style::ShapeStyle;

#[derive(Model, GetterMethods)]
pub struct RectangleEntity {
    rectangle: Rectangle,
    style: ShapeStyle,
}

impl RectangleEntity {
    pub fn new(id: impl Id + 'static, rectangle: Rectangle, style: ShapeStyle) -> Entity {
        let mut entity = Entity::new(
            id,
            RectangleEntity {
                rectangle,
                style,
            }
        );

	entity.add_feature(move_draw());
	entity.add_feature(render());

        entity
    }

    pub fn move_draw() -> MoveDraw {
        MoveDraw {
	    mouse_down: |entity, current_point| {
                let rectangle: &mut RectangleEntity = entity.model_ref_mut();
                rectangle.rectangle.drag(current_point)
	    },
	    mouse_move: |entity, start, current_point| {
                let rectangle: &mut RectangleEntity = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
	    },
	    mouse_up: |entity, start, current_point| {
                let rectangle: &mut RectangleEntity = entity.model_ref_mut();

                let delta: Point = current_point - start;
                rectangle.rectangle.resize(delta.x(), delta.y());
	    },
        }
    }

    pub fn render() -> Render {
	Render {
            render: |entity: &Entity, renderer: &mut dyn Renderer| {
                let rectangle: &RectangleEntity = entity.model_ref();

                renderer.rectangle(rectangle.rectangle(), &rectangle.style);
            },
        }
    }
}
