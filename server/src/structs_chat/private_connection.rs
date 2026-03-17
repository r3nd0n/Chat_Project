use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrivateChat {
    pub from: String,
    pub to: String,
    pub message: String,
    pub timestamp: u64,
}

impl PrivateChat {
    pub fn new(from: String, to: String, message: String, timestamp: u64) -> Self {
        Self {
            from,
            to,
            message,
            timestamp,
        }
    }
}
