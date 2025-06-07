use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Segment<P> {
    start: P,
    end: P,
}

impl<P> Segment<P> {
    pub fn new(start: P, end: P) -> Self {
        Self { start, end }
    }

    pub fn intersects_segment(&self, other: &Self) -> bool {
        todo!();
    }
}
