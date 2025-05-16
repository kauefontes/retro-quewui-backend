use sqlx::{Pool, Sqlite};
use anyhow::Result;
use serde_json;
use uuid::Uuid;

// Generic repository trait for database operations
pub trait Repository<T> {
    fn new(pool: Pool<Sqlite>) -> Self;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn find_by_id(&self, id: &str) -> Result<Option<T>>;
    async fn create(&self, item: T) -> Result<T>;
    async fn update(&self, id: &str, item: T) -> Result<T>;
    async fn delete(&self, id: &str) -> Result<bool>;
}

// Helper function to convert Vec<String> to JSON string
pub fn vec_to_json<T: serde::Serialize>(vec: &[T]) -> Result<String> {
    Ok(serde_json::to_string(vec)?)
}

// Helper function to convert JSON string to Vec<String>
pub fn json_to_vec<T: serde::de::DeserializeOwned>(json: &str) -> Result<Vec<T>> {
    Ok(serde_json::from_str(json)?)
}

// Generate a new UUID
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}
