// filepath: /home/kaue/developer/quewuicom/retro-quewui-backend/src/models/contact.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};

use crate::error::{AppResult, AppError};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
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

pub struct ContactRepository {
    db: Pool<Sqlite>,
}

impl ContactRepository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }

    pub async fn create(&self, contact: ContactMessage) -> AppResult<ContactMessage> {
        let result = sqlx::query!(
            r#"
            INSERT INTO contacts (id, name, email, message, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, email, message, created_at
            "#,
            contact.id,
            contact.name,
            contact.email,
            contact.message,
            contact.created_at,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        Ok(ContactMessage {
            id: result.id.expect("ID should not be null"),
            name: result.name,
            email: result.email,
            message: result.message,
            created_at: DateTime::from_naive_utc_and_offset(
                result.created_at.expect("Created at should not be null"), 
                Utc
            ),
        })
    }

    pub async fn get_all(&self) -> AppResult<Vec<ContactMessage>> {
        let results = sqlx::query!(
            r#"
            SELECT id, name, email, message, created_at
            FROM contacts
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        Ok(results
            .into_iter()
            .map(|row| ContactMessage {
                id: row.id.expect("ID should not be null"),
                name: row.name,
                email: row.email,
                message: row.message,
                created_at: DateTime::from_naive_utc_and_offset(
                    row.created_at.expect("Created at should not be null"), 
                    Utc
                ),
            })
            .collect())
    }

    pub async fn get_by_id(&self, id: &str) -> AppResult<ContactMessage> {
        let result = sqlx::query!(
            r#"
            SELECT id, name, email, message, created_at
            FROM contacts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFoundError(format!("Contact not found with id: {}", id)),
            _ => AppError::DatabaseError(e),
        })?;

        Ok(ContactMessage {
            id: result.id.expect("ID should not be null"),
            name: result.name,
            email: result.email,
            message: result.message,
            created_at: DateTime::from_naive_utc_and_offset(
                result.created_at.expect("Created at should not be null"), 
                Utc
            ),
        })
    }
    
    pub async fn delete(&self, id: &str) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM contacts
            WHERE id = $1
            RETURNING id
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;
        
        match result {
            Some(_) => Ok(()),
            None => Err(AppError::NotFoundError(format!("Contact not found with id: {}", id))),
        }
    }
}
