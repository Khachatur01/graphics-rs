pub trait Identifier: Clone + ToString {
    fn generate() -> Self;
}
