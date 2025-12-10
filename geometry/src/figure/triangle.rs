use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct Triangle<P> {
    p0: P,
    p1: P,
    p2: P,
}
