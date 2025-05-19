use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "project-1",
    "title": "Automotive Dashboard",
    "description": "Real-time dashboard for vehicle diagnostics and monitoring",
    "technologies": ["React", "TypeScript", "WebSockets", "D3.js"],
    "github_url": "https://github.com/username/auto-dashboard",
    "live_url": "https://auto-dashboard.example.com",
    "image_url": "https://example.com/images/project1.jpg",
    "image_urls": ["https://example.com/images/project1-detail1.jpg", "https://example.com/images/project1-detail2.jpg"],
    "year": 2024,
    "highlights": ["Real-time data visualization", "Cross-platform compatibility"]
}))]
pub struct Project {
    /// Unique identifier for the project
    pub id: String,
    /// Project title
    pub title: String,
    /// Detailed description of the project
    pub description: String,
    /// List of technologies used in the project
    pub technologies: Vec<String>,
    /// Optional link to GitHub repository
    pub github_url: Option<String>,
    /// Optional link to live demo
    pub live_url: Option<String>,
    /// Optional URL to the main project image
    pub image_url: Option<String>,
    /// Optional list of additional project image URLs
    pub image_urls: Option<Vec<String>>,
    /// Year the project was completed
    pub year: i32,
    /// Key highlights or features of the project
    pub highlights: Vec<String>,
}

impl Project {
    pub fn new(
        title: String,
        description: String,
        technologies: Vec<String>,
        github_url: Option<String>,
        live_url: Option<String>,
        image_url: Option<String>,
        image_urls: Option<Vec<String>>,
        year: i32,
        highlights: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            technologies,
            github_url,
            live_url,
            image_url,
            image_urls,
            year,
            highlights,
        }
    }
}

// No more mock data - using database instead
