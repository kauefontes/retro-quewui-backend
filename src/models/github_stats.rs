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

// Mock data for development
pub fn get_mock_github_stats() -> GithubStats {
    GithubStats {
        username: "quewui".to_string(),
        repo_count: 42,
        followers: 128,
        contributions: 829,
        top_languages: vec![
            TopLanguage {
                name: "TypeScript".to_string(),
                percentage: 35,
            },
            TopLanguage {
                name: "Rust".to_string(),
                percentage: 25,
            },
            TopLanguage {
                name: "JavaScript".to_string(),
                percentage: 20,
            },
            TopLanguage {
                name: "Python".to_string(),
                percentage: 15,
            },
            TopLanguage {
                name: "CSS".to_string(),
                percentage: 5,
            },
        ],
        recent_activity: vec![
            RecentActivity {
                date: "2025-05-10".to_string(),
                message: "Fixed memory leak in background processing".to_string(),
                repo: "quewui/rust-performance".to_string(),
            },
            RecentActivity {
                date: "2025-05-08".to_string(),
                message: "Added dark mode support".to_string(),
                repo: "quewui/retro-quewui".to_string(),
            },
            RecentActivity {
                date: "2025-05-05".to_string(),
                message: "Implemented WebAssembly optimizations".to_string(),
                repo: "quewui/wasm-experiments".to_string(),
            },
            RecentActivity {
                date: "2025-05-01".to_string(),
                message: "Updated README with installation instructions".to_string(),
                repo: "quewui/developer-tools".to_string(),
            },
            RecentActivity {
                date: "2025-04-28".to_string(),
                message: "Fixed CI pipeline configuration".to_string(),
                repo: "quewui/github-actions-templates".to_string(),
            },
        ],
    }
}
