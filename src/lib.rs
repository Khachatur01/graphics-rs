use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Request {
    Create {
        logical_type: String,
        /* todo */
    },
    Update,
    Delete,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok,
    Error,
}

pub struct Output {
    scene: String,
    metadata: String,
}

pub struct Scene {

}

impl Scene {
    pub fn new() -> Self {
        Self {}
    }

    pub fn request(&self, request: Request) -> Response {
        Response::Ok
    }

    pub fn convert_to(&self) -> Output {
        todo!()
    }
}
