use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john.doe@example.com",
    "message": "This is a test message",
    "created_at": "2023-01-01T12:00:00Z"
}))]
pub struct ContactMessage {
    /// Unique identifier for this message
    pub id: String,
    /// Sender's full name
    pub name: String,
    /// Sender's email address
    pub email: String,
    /// Content of the message
    pub message: String,
    /// When the message was created
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ContactResponse {
    /// Indicates if the operation was successful
    pub success: bool,
    /// Response message with details about the operation
    pub message: String,
}

impl ContactMessage {
    pub fn new(name: String, email: String, message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            email,
            message,
            created_at: Utc::now(),
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
