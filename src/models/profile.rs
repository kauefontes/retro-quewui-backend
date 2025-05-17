use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "bio": [
        "Senior Software Developer specializing in creating intuitive and high-performance web applications.",
        "I'm passionate about building software that combines technical excellence with great user experience."
    ],
    "social_links": [
        {"title": "GitHub", "url": "https://github.com/kauefontes", "icon": "󰊤"},
        {"title": "LinkedIn", "url": "https://www.linkedin.com/in/kauefontes/", "icon": "󰌻"}
    ],
    "education": [
        {"degree": "Computer Science, B.Sc.", "institution": "University of Technology", "period": "2016 - 2020"}
    ],
    "languages": [
        {"name": "English", "level": "Fluent"},
        {"name": "Portuguese", "level": "Native"}
    ]
}))]
pub struct Profile {
    pub bio: Vec<String>,
    pub social_links: Vec<SocialLink>,
    pub education: Vec<Education>,
    pub languages: Vec<Language>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "title": "GitHub",
    "url": "https://github.com/kauefontes",
    "icon": "󰊤"
}))]
pub struct SocialLink {
    pub title: String,
    pub url: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "degree": "Computer Science, B.Sc.",
    "institution": "University of Technology",
    "period": "2016 - 2020"
}))]
pub struct Education {
    pub degree: String,
    pub institution: String,
    pub period: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "name": "English",
    "level": "Fluent"
}))]
pub struct Language {
    pub name: String,
    pub level: String,
}

pub fn get_mock_profile() -> Profile {
    Profile {
        bio: vec![
            "Senior Software Developer specializing in creating intuitive and high-performance web applications. With over 5 years of professional experience, I've worked on projects ranging from automotive dashboards to IoT systems and developer tools.".to_string(),
            "I'm passionate about building software that combines technical excellence with great user experience. My approach emphasizes clean code, performance optimization, and sustainable architecture that can evolve with changing requirements.".to_string(),
            "When I'm not coding, you can find me exploring new technologies, contributing to open-source projects, or enjoying outdoor activities.".to_string(),
        ],
        social_links: vec![
            SocialLink {
                title: "GitHub".to_string(),
                url: "https://github.com/kauefontes".to_string(),
                icon: "󰊤".to_string(),
            },
            SocialLink {
                title: "LinkedIn".to_string(),
                url: "https://www.linkedin.com/in/kauefontes/".to_string(),
                icon: "󰌻".to_string(),
            },
        ],
        education: vec![
            Education {
                degree: "Computer Science, B.Sc.".to_string(),
                institution: "University of Technology".to_string(),
                period: "2016 - 2020".to_string(),
            },
            Education {
                degree: "Advanced Software Architecture".to_string(),
                institution: "Tech Institute".to_string(),
                period: "2022".to_string(),
            },
        ],
        languages: vec![
            Language {
                name: "English".to_string(),
                level: "Fluent".to_string(),
            },
            Language {
                name: "Portuguese".to_string(),
                level: "Native".to_string(),
            },
            Language {
                name: "Spanish".to_string(),
                level: "Intermediate".to_string(),
            },
        ],
    }
}
