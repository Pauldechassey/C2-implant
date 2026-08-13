use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub id: i32,
    pub command: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommandResponse {
    pub output: String,
    pub status: String,
}