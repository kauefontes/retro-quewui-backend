use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Skill {
    pub category: String,
    pub items: Vec<String>,
}

// No more mock data - using database instead
