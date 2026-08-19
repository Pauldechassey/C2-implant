use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: Uuid,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub output: String,
}