// filepath: /home/kaue/developer/quewuicom/retro-quewui-backend/src/models/profile.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SocialLink {
    pub title: String,
    pub url: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Education {
    pub degree: String,
    pub institution: String,
    pub period: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Language {
    pub name: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Profile {
    pub bio: Vec<String>,
    pub social_links: Vec<SocialLink>,
    pub education: Vec<Education>,
    pub languages: Vec<Language>,
}

// Mock data for development
pub fn get_mock_profile() -> Profile {
    Profile {
        bio: vec![
            "I'm a passionate software engineer with over 7 years of experience developing for the web and mobile platforms.".to_string(),
            "Specialized in React, TypeScript, and Rust, I build performant applications with clean, maintainable code.".to_string(),
            "I'm particularly interested in WebAssembly, system programming, and creating retro-inspired interfaces with modern technology.".to_string(),
        ],
        social_links: vec![
            SocialLink {
                title: "GitHub".to_string(),
                url: "https://github.com/quewui".to_string(),
                icon: "github".to_string(),
            },
            SocialLink {
                title: "LinkedIn".to_string(),
                url: "https://linkedin.com/in/quewui".to_string(),
                icon: "linkedin".to_string(),
            },
            SocialLink {
                title: "Twitter".to_string(),
                url: "https://twitter.com/quewui".to_string(),
                icon: "twitter".to_string(),
            },
        ],
        education: vec![
            Education {
                degree: "Master of Computer Science".to_string(),
                institution: "University of Technology".to_string(),
                period: "2019-2021".to_string(),
            },
            Education {
                degree: "Bachelor of Software Engineering".to_string(),
                institution: "Technical Institute".to_string(),
                period: "2015-2019".to_string(),
            },
        ],
        languages: vec![
            Language {
                name: "English".to_string(),
                level: "Native".to_string(),
            },
            Language {
                name: "Spanish".to_string(),
                level: "Fluent".to_string(),
            },
            Language {
                name: "French".to_string(),
                level: "Intermediate".to_string(),
            },
        ],
    }
}
