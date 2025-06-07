use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Polyline<P> {
    vertices: Vec<P>,
}

impl<P> Polyline<P> {
    pub fn new(vertices: Vec<P>) -> Self {
        Self { vertices }
    }
}
