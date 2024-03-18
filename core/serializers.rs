use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct TaskCreateSerializer {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskUpdateSerializer {
    pub title: String,
}
