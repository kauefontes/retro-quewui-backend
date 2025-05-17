// filepath: /home/kaue/developer/quewuicom/retro-quewui-backend/src/routes/github_stats.rs
use actix_web::{get, put, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::auth::AuthenticatedUser;
use crate::error::AppResult;
use crate::models::github_stats::get_mock_github_stats;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateGithubStatsRequest {
    /// GitHub username
    pub username: Option<String>,
    /// Number of public repositories
    pub repo_count: Option<i32>,
    /// Number of followers
    pub followers: Option<i32>,
    /// Number of contributions in past year
    pub contributions: Option<i32>,
}

/// Get GitHub statistics
///
/// Returns the GitHub statistics for the portfolio owner.
#[utoipa::path(
    get,
    path = "/github-stats",
    tag = "github-stats",
    responses(
        (status = 200, description = "GitHub statistics retrieved successfully", body = GithubStats),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/github-stats")]
pub async fn get_github_stats() -> impl Responder {
    let stats = get_mock_github_stats();
    HttpResponse::Ok().json(stats)
}

/// Update GitHub statistics
///
/// Updates the GitHub statistics for the portfolio owner.
/// Requires authentication.
#[utoipa::path(
    put,
    path = "/github-stats",
    tag = "github-stats",
    security(
        ("jwt_auth" = [])
    ),
    request_body = UpdateGithubStatsRequest,
    responses(
        (status = 200, description = "GitHub statistics updated successfully", body = GithubStats),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/github-stats")]
pub async fn update_github_stats(
    stats_req: web::Json<UpdateGithubStatsRequest>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    // Get current stats
    let mut stats = get_mock_github_stats();
    
    // Update fields if provided
    if let Some(username) = &stats_req.username {
        stats.username = username.clone();
    }
    
    if let Some(repo_count) = stats_req.repo_count {
        stats.repo_count = repo_count;
    }
    
    if let Some(followers) = stats_req.followers {
        stats.followers = followers;
    }
    
    if let Some(contributions) = stats_req.contributions {
        stats.contributions = contributions;
    }
    
    // In a real application, you would save this to a database
    info!("Updated GitHub stats for user: {}", stats.username);
    
    Ok(HttpResponse::Ok().json(stats))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_github_stats)
       .service(update_github_stats);
}
