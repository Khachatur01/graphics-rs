use crate::figure::path::command::arc_to::ArcTo;
use crate::figure::path::command::bezier_to::BezierTo;
use crate::figure::path::command::horizontal_line_to::HorizontalLineTo;
use crate::figure::path::command::line_to::LineTo;
use crate::figure::path::command::move_to::MoveTo;
use crate::figure::path::command::vertical_line_to::VerticalLineTo;
use crate::figure::point::Point;
use crate::math::Drag;

mod arc_to;
mod bezier_to;
mod horizontal_line_to;
mod line_to;
mod move_to;
mod positioning;
mod vertical_line_to;

pub enum Command {
    MoveTo(MoveTo),

    LineTo(LineTo),
    HorizontalLineTo(HorizontalLineTo),
    VerticalLineTo(VerticalLineTo),

    BezierTo(BezierTo),

    ArcTo(ArcTo),

    Close,
}

impl Drag for Command {
    fn drag(&mut self, delta: &Point) {
        match self {
            Command::MoveTo(move_to) => move_to.drag(delta),
            Command::LineTo(line_to) => todo!(),
            Command::HorizontalLineTo(horizontal_line_to) => todo!(),
            Command::VerticalLineTo(vertical_line_to) => todo!(),
            Command::BezierTo(bezier_to) => todo!(),
            Command::ArcTo(arc_to) => todo!(),
            Command::Close => {}
        }
    }
}
