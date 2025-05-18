use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "post-1",
    "title": "The Future of Rust in Web Development",
    "date": "2025-04-10",
    "tags": ["Rust", "Web Development", "Backend"],
    "excerpt": "Exploring how Rust is changing the landscape of web development with its performance and safety guarantees.",
    "content": "# The Future of Rust in Web Development\n\nAs web applications become more complex..."
}))]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub excerpt: String,
    pub content: String,
}

impl Post {
    pub fn new(
        title: String,
        date: String,
        tags: Vec<String>,
        excerpt: String,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            date,
            tags,
            excerpt,
            content,
        }
    }
}

// No more mock data - using database instead
