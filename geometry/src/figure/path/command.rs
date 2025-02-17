use crate::figure::path::command::arc_to::ArcTo;
use crate::figure::path::command::bezier_to::BezierTo;
use crate::figure::path::command::horizontal_line_to::HorizontalLineTo;
use crate::figure::path::command::line_to::LineTo;
use crate::figure::path::command::move_to::MoveTo;
use crate::figure::path::command::vertical_line_to::VerticalLineTo;

mod move_to;
mod line_to;
mod horizontal_line_to;
mod vertical_line_to;
mod arc_to;
mod positioning;
mod bezier_to;

pub enum Command {
    MoveTo(MoveTo),

    LineTo(LineTo),
    HorizontalLineTo(HorizontalLineTo),
    VerticalLineTo(VerticalLineTo),

    BezierTo(BezierTo),

    ArcTo(ArcTo),

    Close,
}
