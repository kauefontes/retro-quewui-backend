use serde::{Deserialize, Serialize};
use serde_json::json;
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

// Mock data for development
pub fn get_mock_experiences() -> Vec<Experience> {
    vec![
        Experience {
            id: "exp-1".to_string(),
            company: "Tech Innovations Inc.".to_string(),
            position: "Senior Frontend Engineer".to_string(),
            start_date: "2023-01".to_string(),
            end_date: None,
            description: "Leading frontend development for enterprise applications.".to_string(),
            technologies: vec![
                "React".to_string(),
                "TypeScript".to_string(),
                "GraphQL".to_string(),
                "Tailwind CSS".to_string(),
            ],
            highlights: vec![
                "Implemented design system used across 5 products".to_string(),
                "Reduced application bundle size by 40%".to_string(),
                "Mentored junior developers and led technical interviews".to_string(),
                "Introduced automated testing, achieving 85% coverage".to_string(),
            ],
        },
        Experience {
            id: "exp-2".to_string(),
            company: "Digital Solutions LLC".to_string(),
            position: "Software Engineer".to_string(),
            start_date: "2021-03".to_string(),
            end_date: Some("2022-12".to_string()),
            description: "Full-stack development for SaaS applications.".to_string(),
            technologies: vec![
                "JavaScript".to_string(),
                "Node.js".to_string(),
                "Express".to_string(),
                "MongoDB".to_string(),
            ],
            highlights: vec![
                "Developed RESTful APIs for client applications".to_string(),
                "Implemented real-time features using WebSockets".to_string(),
                "Optimized database queries, improving performance by 30%".to_string(),
                "Led migration from monolith to microservices architecture".to_string(),
            ],
        },
        Experience {
            id: "exp-3".to_string(),
            company: "Innovative Startups Co.".to_string(),
            position: "Junior Developer".to_string(),
            start_date: "2019-06".to_string(),
            end_date: Some("2021-02".to_string()),
            description: "Frontend development for consumer web applications.".to_string(),
            technologies: vec![
                "HTML/CSS".to_string(),
                "JavaScript".to_string(),
                "React".to_string(),
                "Bootstrap".to_string(),
            ],
            highlights: vec![
                "Built responsive UIs for web and mobile".to_string(),
                "Participated in agile development process".to_string(),
                "Collaborated with designers to implement UI components".to_string(),
                "Assisted in maintaining CI/CD pipelines".to_string(),
            ],
        },
    ]
}
