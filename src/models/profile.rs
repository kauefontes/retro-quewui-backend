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

// No more mock data - using database instead
