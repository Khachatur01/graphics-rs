use crate::model_2d::path::command::arc_to::ArcTo;
use crate::model_2d::path::command::cubic_to::CubicTo;
use crate::model_2d::path::command::horizontal_line_to::HorizontalLineTo;
use crate::model_2d::path::command::line_to::LineTo;
use crate::model_2d::path::command::move_to::MoveTo;
use crate::model_2d::path::command::quadratic_to::QuadraticTo;
use crate::model_2d::path::command::short_cubic_to::ShortCubicTo;
use crate::model_2d::path::command::short_quadratic_to::ShortQuadraticTo;
use crate::model_2d::path::command::vertical_line_to::VerticalLineTo;

mod move_to;
mod line_to;
mod horizontal_line_to;
mod vertical_line_to;
mod quadratic_to;
mod short_quadratic_to;
mod cubic_to;
mod short_cubic_to;
mod arc_to;

pub enum Command {
    MoveTo(MoveTo),

    LineTo(LineTo),
    HorizontalLineTo(HorizontalLineTo),
    VerticalLineTo(VerticalLineTo),

    QuadraticTo(QuadraticTo),
    ShortQuadraticTo(ShortQuadraticTo),

    CubicTo(CubicTo),
    ShortCubicTo(ShortCubicTo),

    ArcTo(ArcTo),

    Close,
}
