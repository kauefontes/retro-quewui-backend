use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactMessage {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
}

impl ContactMessage {
    pub fn new(name: String, email: String, message: String) -> Self {
        Self {
            name,
            email,
            message,
        }
    }
}

impl ContactResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: "Message sent successfully".to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
        }
    }
}
