use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub category: String,
    pub items: Vec<String>,
}

// Mock data for development
pub fn get_mock_skills() -> Vec<Skill> {
    vec![
        Skill {
            category: "Languages".to_string(),
            items: vec![
                "JavaScript".to_string(),
                "TypeScript".to_string(),
                "Rust".to_string(),
                "Python".to_string(),
                "SQL".to_string(),
            ],
        },
        Skill {
            category: "Frontend".to_string(),
            items: vec![
                "React".to_string(),
                "Vue.js".to_string(),
                "HTML5".to_string(),
                "CSS3/SASS".to_string(),
                "Tailwind CSS".to_string(),
            ],
        },
        Skill {
            category: "Backend".to_string(),
            items: vec![
                "Node.js".to_string(),
                "Express".to_string(),
                "Actix Web".to_string(),
                "Django".to_string(),
                "GraphQL".to_string(),
            ],
        },
        Skill {
            category: "DevOps & Tools".to_string(),
            items: vec![
                "Docker".to_string(),
                "Git".to_string(),
                "GitHub Actions".to_string(),
                "AWS".to_string(),
                "Linux".to_string(),
            ],
        },
    ]
}
