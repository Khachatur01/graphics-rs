use crate::model_2d::path::command::arc_to::ArcTo;
use crate::model_2d::path::command::bezier_to::BezierTo;
use crate::model_2d::path::command::horizontal_line_to::HorizontalLineTo;
use crate::model_2d::path::command::line_to::LineTo;
use crate::model_2d::path::command::move_to::MoveTo;
use crate::model_2d::path::command::vertical_line_to::VerticalLineTo;

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
