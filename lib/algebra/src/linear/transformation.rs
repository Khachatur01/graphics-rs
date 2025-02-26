use getter_methods::GetterMethods;

#[derive(GetterMethods, Copy, Clone)]
pub struct ReferencePoint {
    x: f64,
    y: f64,
}

#[derive(GetterMethods, Copy, Clone)]
pub struct Rotation {
    angle: f64,
    reference_point: ReferencePoint,
}

#[derive(GetterMethods, Copy, Clone)]
pub struct Skew {
    x_angle: f64,
    y_angle: f64,
    reference_point: ReferencePoint,
}

#[derive(GetterMethods, Copy, Clone)]
pub struct Scale {
    by_x: f64,
    by_y: f64,
    reference_point: ReferencePoint,
}

#[derive(GetterMethods, Copy, Clone)]
pub struct Matrix {
    /* todo */
}

#[derive(Copy, Clone)]
pub enum Transformation {
    Rotation(Rotation),
    Skew(Skew),
    Scale(Scale),
    Matrix(Matrix),
}
