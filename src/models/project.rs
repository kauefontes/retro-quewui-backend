use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
    pub year: i32,
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
