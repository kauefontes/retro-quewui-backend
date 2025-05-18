// filepath: /home/kaue/developer/quewuicom/retro-quewui-backend/src/models/github_stats.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TopLanguage {
    pub name: String,
    pub percentage: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct RecentActivity {
    pub date: String,
    pub message: String,
    pub repo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GithubStats {
    pub username: String,
    pub repo_count: i32,
    pub followers: i32,
    pub contributions: i32,
    pub top_languages: Vec<TopLanguage>,
    pub recent_activity: Vec<RecentActivity>,
}

// No more mock data - using database instead
