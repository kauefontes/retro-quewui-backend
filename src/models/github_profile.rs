use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubProfile {
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub blog: Option<String>,
    pub twitter_username: Option<String>,
    pub company: Option<String>,
    pub followers: i32,
    pub following: i32,
    pub public_repos: i32,
    pub public_gists: i32,
    pub html_url: String,
    pub created_at: String,
    pub organizations: Vec<GitHubOrganization>,
    pub top_repositories: Vec<GitHubRepository>,
    pub recent_activity: Vec<GitHubActivityItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubOrganization {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub description: Option<String>,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubRepository {
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: i32,
    pub forks_count: i32,
    pub topics: Vec<String>,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubActivityItem {
    pub event_type: String,
    pub repo_name: String,
    pub repo_url: String,
    pub created_at: String,
    pub details: serde_json::Value,
}
