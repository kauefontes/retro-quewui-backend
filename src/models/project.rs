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
    "image_url": null,
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
    /// Optional URL to project image/screenshot
    pub image_url: Option<String>,
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
            year,
            highlights,
        }
    }
}

// Mock data for development
pub fn get_mock_projects() -> Vec<Project> {
    vec![
        Project {
            id: "project-1".to_string(),
            title: "Automotive Dashboard".to_string(),
            description: "Real-time dashboard for vehicle diagnostics and monitoring".to_string(),
            technologies: vec![
                "React".to_string(),
                "TypeScript".to_string(),
                "WebSockets".to_string(),
                "D3.js".to_string(),
            ],
            github_url: Some("https://github.com/username/auto-dashboard".to_string()),
            live_url: Some("https://auto-dashboard.example.com".to_string()),
            image_url: None,
            year: 2024,
            highlights: vec![
                "Real-time data visualization".to_string(),
                "Cross-platform compatibility".to_string(),
                "Customizable widgets and layouts".to_string(),
                "Low-latency performance optimizations".to_string(),
            ],
        },
        Project {
            id: "project-2".to_string(),
            title: "Developer Terminal Portfolio".to_string(),
            description: "Interactive terminal-style portfolio website with vim-like navigation"
                .to_string(),
            technologies: vec![
                "React".to_string(),
                "Vite".to_string(),
                "Zustand".to_string(),
                "Tailwind CSS".to_string(),
            ],
            github_url: Some("https://github.com/username/terminal-portfolio".to_string()),
            live_url: Some("https://terminal-portfolio.example.com".to_string()),
            image_url: None,
            year: 2025,
            highlights: vec![
                "Retro terminal UI with NEON-inspired aesthetics".to_string(),
                "Vim-like keyboard navigation".to_string(),
                "Responsive design for all device sizes".to_string(),
                "Animated terminal boot sequence".to_string(),
            ],
        },
        Project {
            id: "project-3".to_string(),
            title: "Garden Monitoring System".to_string(),
            description: "IoT solution for monitoring and automating garden care".to_string(),
            technologies: vec![
                "Rust".to_string(),
                "ESP32".to_string(),
                "MQTT".to_string(),
                "React Native".to_string(),
            ],
            github_url: Some("https://github.com/username/garden-monitor".to_string()),
            live_url: None,
            image_url: None,
            year: 2023,
            highlights: vec![
                "Low-power sensor network".to_string(),
                "Automated watering system".to_string(),
                "Weather data integration".to_string(),
                "Mobile app for remote monitoring".to_string(),
            ],
        },
    ]
}
