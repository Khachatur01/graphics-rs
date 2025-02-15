use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct ElementId {
    owner_id: usize,
    index: usize,
}
