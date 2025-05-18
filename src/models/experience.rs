use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "exp-1",
    "company": "Tech Innovations Inc.",
    "position": "Senior Frontend Engineer",
    "start_date": "2023-01",
    "end_date": null,
    "description": "Leading frontend development for enterprise applications.",
    "technologies": ["React", "TypeScript", "GraphQL", "Tailwind CSS"],
    "highlights": ["Implemented design system used across 5 products", "Reduced application bundle size by 40%"]
}))]
pub struct Experience {
    /// Unique identifier for the experience
    pub id: String,
    /// Name of the company or organization
    pub company: String,
    /// Job title or position held
    pub position: String,
    /// When the position started (format: YYYY-MM)
    pub start_date: String,
    /// When the position ended (format: YYYY-MM), null if current position
    pub end_date: Option<String>, // None means present
    /// Detailed description of the role and responsibilities
    pub description: String,
    /// List of technologies and tools used in this role
    pub technologies: Vec<String>,
    /// Key achievements and notable contributions
    pub highlights: Vec<String>,
}

impl Experience {
    pub fn new(
        company: String,
        position: String,
        start_date: String,
        end_date: Option<String>,
        description: String,
        technologies: Vec<String>,
        highlights: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            company,
            position,
            start_date,
            end_date,
            description,
            technologies,
            highlights,
        }
    }
}

// No more mock data - using database instead
