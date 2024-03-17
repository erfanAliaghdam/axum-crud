use serde::Serialize;


#[derive(Serialize)]
pub struct Message {
    pub status: &'static str,
    pub message: &'static str,
}
