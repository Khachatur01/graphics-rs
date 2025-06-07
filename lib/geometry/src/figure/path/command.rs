use crate::figure::path::command::arc_to::ArcTo;
use crate::figure::path::command::bezier_to::BezierTo;
use crate::figure::path::command::horizontal_line_to::HorizontalLineTo;
use crate::figure::path::command::line_to::LineTo;
use crate::figure::path::command::move_to::MoveTo;
use crate::figure::path::command::vertical_line_to::VerticalLineTo;
use serde::{Deserialize, Serialize};

pub mod arc_to;
pub mod bezier_to;
pub mod horizontal_line_to;
pub mod line_to;
pub mod move_to;
pub mod positioning;
pub mod vertical_line_to;

#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
    MoveTo(MoveTo),

    LineTo(LineTo),
    HorizontalLineTo(HorizontalLineTo),
    VerticalLineTo(VerticalLineTo),

    BezierTo(BezierTo),

    ArcTo(ArcTo),

    Close,
}
