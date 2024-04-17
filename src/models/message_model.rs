use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MessageMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error: String,
}
