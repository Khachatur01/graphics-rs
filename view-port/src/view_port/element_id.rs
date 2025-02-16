use getter_methods::GetterMethods;

#[derive(GetterMethods)]
pub struct ElementId {
    owner_id: usize,
    index: usize,
}

impl ElementId {
    pub fn new(owner_id: usize, index: usize) -> Self {
        Self { owner_id, index }
    }
}
