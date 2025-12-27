use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicUsize;

static PARAM_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Id(usize);

impl Id {
    pub fn generate() -> Self {
        let id: usize = PARAM_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Id(id)
    }
}
