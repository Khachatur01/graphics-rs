mod render;
mod draw;

use crate::geometric::rectangle_view::draw::{mouse_down, mouse_move, mouse_up};
use crate::geometric::rectangle_view::render::render;
use element_view::{DefaultWithId, Element, ElementView};
use geometry::figure::rectangle::Rectangle;
use getter_methods::GetterMethods;
use interactivity::tool::draw_tool::draw_mode::MoveDraw;
use rendering::style::shape_style::ShapeStyle;
use rendering::Render;
use std::any::Any;

#[derive(GetterMethods)]
pub struct RectangleElement {
    rectangle: Rectangle,
    style: ShapeStyle,
    // behaviors: Vec<Box<dyn Any>>,
}

impl RectangleElement {
    pub fn new<Id: 'static>(id: Id, rectangle: Rectangle, style: ShapeStyle) -> Element<Id, RectangleElement> {
        Element::new(
            id,
            RectangleElement {
                rectangle,
                style,
            },
            vec![
                Box::new(Render { render: render::<Id> }),
                Box::new(MoveDraw {
                    mouse_down: mouse_down::<Id>,
                    mouse_move: mouse_move::<Id>,
                    mouse_up: mouse_up::<Id>,
                })
            ]
        )
    }
    // pub fn new(id: Id, rectangle: Rectangle, style: ShapeStyle) -> Element<Id> {
    //     Self {
    //         id,
    //         rectangle,
    //         style,
    //         behaviors: vec![
    //             Box::new(Render { render: render::<Id> }),
    //             Box::new(MoveDraw {
    //                 mouse_down: mouse_down::<Id>,
    //                 mouse_move: mouse_move::<Id>,
    //                 mouse_up: mouse_up::<Id>,
    //             })
    //         ],
    //     }
    // }
}
//
// impl<Id> ElementView<Id> for RectangleElement<Id> {
//     fn id(&self) -> &Id {
//         &self.id
//     }
// }

// impl<Id: 'static> DefaultWithId<Id> for RectangleElement {
//     fn default_with_id(id: Id) -> Self {
//         Self::new(
//             id,
//             Rectangle::zero_sized(Point::default()),
//             ShapeStyle::default(),
//         )
//     }
// }
