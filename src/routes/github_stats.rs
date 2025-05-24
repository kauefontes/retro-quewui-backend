use actix_web::{get, put, web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;

use crate::auth::AuthenticatedUser;
use crate::config::database::DbPool;
use crate::error::{AppError, AppResult};
use crate::models::github_stats::{GithubStats, TopLanguage, RecentActivity};
use crate::models::github_stats_repository::GithubStatsRepository;
use crate::models::repository::Repository;
use crate::services::github_service::GitHubService;

// Cache para os dados de GitHub, com um tempo de expiração
static GITHUB_STATS_CACHE: Lazy<Mutex<Option<(Instant, GithubStats)>>> = Lazy::new(|| Mutex::new(None));
static CACHE_EXPIRATION: Duration = Duration::from_secs(3600); // 1 hora

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
    /// Top programming languages
    pub top_languages: Option<Vec<TopLanguage>>,
    /// Recent GitHub activity
    pub recent_activity: Option<Vec<RecentActivity>>,
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
pub async fn get_github_stats(
    db: web::Data<DbPool>,
) -> AppResult<impl Responder> {
    let repo = GithubStatsRepository::new(db.get_ref().clone());
    
    // Verificar se temos um cache válido
    {
        let cache_lock = GITHUB_STATS_CACHE.lock().unwrap();
        if let Some((timestamp, cached_stats)) = &*cache_lock {
            if timestamp.elapsed() < CACHE_EXPIRATION {
                // Cache válido, use-o
                info!("Usando cache para GitHub stats (expiração em {} segundos)", 
                      CACHE_EXPIRATION.as_secs() - timestamp.elapsed().as_secs());
                return Ok(HttpResponse::Ok().json(cached_stats.clone()));
            }
            // Cache expirado, atualize-o
            info!("Cache expirado, obtendo novos dados");
        } else {
            info!("Cache vazio, obtendo dados do banco");
        }
    }
    
    // Obtenha os dados do banco de dados
    let stats_list = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch GitHub stats: {}", e);
            AppError::internal_error(format!("Failed to fetch GitHub stats: {}", e))
        })?;
    
    let stats = if stats_list.is_empty() {
        // Create default stats if none exist
        let github_username = std::env::var("GITHUB_USERNAME")
            .unwrap_or_else(|_| "github_user".to_string());
            
        info!("Nenhum dado encontrado no banco, criando stats padrão");
        GithubStats {
            username: github_username,
            repo_count: 0,
            followers: 0,
            contributions: 0,
            top_languages: vec![],
            recent_activity: vec![],
        }
    } else {
        stats_list[0].clone()
    };
    
    // Iniciar uma task em background para atualizar os dados
    // sem bloquear a resposta atual
    let stats_clone = stats.clone();
    let db_clone = db.get_ref().clone();
    actix_web::rt::spawn(async move {
        info!("Iniciando atualização em background dos dados do GitHub");
        let repo = GithubStatsRepository::new(db_clone);
        let github_username = std::env::var("GITHUB_USERNAME")
            .unwrap_or_else(|_| "github_user".to_string());
        let github_token = std::env::var("GITHUB_TOKEN").ok();
        
        let github_service = GitHubService::new(github_username.clone(), github_token);
        let mut stats = stats_clone;
        
        if let Err(e) = update_stats_from_github(&github_service, &repo, &mut stats).await {
            error!("Error updating GitHub stats in background: {}", e);
            return;
        }
        
        // Atualiza o cache com os novos dados
        let mut cache_lock = GITHUB_STATS_CACHE.lock().unwrap();
        *cache_lock = Some((Instant::now(), stats));
        info!("GitHub stats atualizados com sucesso em background");
    });
    
    Ok(HttpResponse::Ok().json(stats))
}

/// Force refresh GitHub statistics
///
/// Forces a refresh of the GitHub statistics from the GitHub API.
/// Requires authentication.
#[utoipa::path(
    get,
    path = "/github-stats/refresh",
    tag = "github-stats",
    security(
        ("jwt_auth" = [])
    ),
    responses(
        (status = 200, description = "GitHub statistics refreshed successfully", body = GithubStats),
        (status = 401, description = "Unauthorized - Invalid or missing authentication token"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/github-stats/refresh")]
pub async fn refresh_github_stats(
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let repo = GithubStatsRepository::new(db.get_ref().clone());
    
    // Get environment variables for GitHub
    let github_username = std::env::var("GITHUB_USERNAME")
        .unwrap_or_else(|_| "github_user".to_string());
    let github_token = std::env::var("GITHUB_TOKEN").ok();
    
    let github_service = GitHubService::new(
        github_username.clone(),
        github_token,
    );
    
    // Get stats from database first
    let stats_list = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch GitHub stats: {}", e);
            AppError::internal_error(format!("Failed to fetch GitHub stats: {}", e))
        })?;
    
    let mut stats = if stats_list.is_empty() {
        // Create default stats if none exist
        GithubStats {
            username: github_username,
            repo_count: 0,
            followers: 0,
            contributions: 0,
            top_languages: vec![],
            recent_activity: vec![],
        }
    } else {
        stats_list[0].clone()
    };

    // Force update from GitHub API
    if let Err(e) = update_stats_from_github(&github_service, &repo, &mut stats).await {
        error!("Error refreshing GitHub stats from API: {}", e);
        return Err(AppError::internal_error(format!("Error refreshing GitHub stats: {}", e)));
    }
    
    info!("GitHub stats refreshed successfully");
    Ok(HttpResponse::Ok().json(stats))
}

async fn update_stats_from_github(
    github_service: &GitHubService,
    repo: &GithubStatsRepository,
    stats: &mut GithubStats,
) -> Result<(), anyhow::Error> {
    // Get profile to update repo count, followers, etc.
    if let Ok(profile) = github_service.get_user_profile().await {
        stats.repo_count = profile.public_repos;
        stats.followers = profile.followers;
        // No direct mapping for contributions in GitHub API
    }
    
    // Update language stats with real data
    if let Ok(languages) = github_service.get_language_stats().await {
        // Convert from service::TopLanguage to models::github_stats::TopLanguage
        stats.top_languages = languages.into_iter()
            .map(|lang| TopLanguage {
                name: lang.name,
                percentage: lang.percentage,
            })
            .collect();
    }
    
    // Update recent activity if available
    if let Ok(activities) = github_service.get_user_activity(5).await {
        stats.recent_activity = activities.into_iter()
            .map(|activity| RecentActivity {
                date: activity.created_at,
                message: format!("{}ed to {}", activity.r#type.to_lowercase(), activity.repo.name),
                repo: activity.repo.name,
            })
            .collect();
    }
    
    // Save to database
    let stats_id = format!("github-stats-{}", 1); // Assuming ID follows a pattern
    repo.update(&stats_id, stats.clone()).await?;
    
    Ok(())
}

// This function is removed as the project uses actix_web's config function instead
// The config function already exists in this file

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
    db: web::Data<DbPool>,
    _user: AuthenticatedUser, // Require authentication
) -> AppResult<impl Responder> {
    let repo = GithubStatsRepository::new(db.get_ref().clone());
    
    // Get all stats
    let stats_list = repo.find_all().await
        .map_err(|e| {
            error!("Failed to fetch GitHub stats: {}", e);
            AppError::internal_error(format!("Failed to fetch GitHub stats: {}", e))
        })?;
    
    let (stats_id, existing_stats) = if stats_list.is_empty() {
        // Create default stats if none exist
        let new_stats = GithubStats {
            username: stats_req.username.clone().unwrap_or_else(|| "github_user".to_string()),
            repo_count: stats_req.repo_count.unwrap_or(0),
            followers: stats_req.followers.unwrap_or(0),
            contributions: stats_req.contributions.unwrap_or(0),
            top_languages: stats_req.top_languages.clone().unwrap_or_default(),
            recent_activity: stats_req.recent_activity.clone().unwrap_or_default(),
        };
        
        let stats = repo.create(new_stats).await
            .map_err(|e| {
                error!("Failed to create GitHub stats: {}", e);
                AppError::internal_error(format!("Failed to create GitHub stats: {}", e))
            })?;
        
        ("github-stats-1".to_string(), stats)
    } else {
        // Use the first stats
        let stats = &stats_list[0];
        let stats_id = format!("github-stats-{}", 1); // Assuming ID follows a pattern
        (stats_id, stats.clone())
    };
    
    // Update with new values if provided
    let updated_stats = GithubStats {
        username: stats_req.username.clone().unwrap_or_else(|| existing_stats.username.clone()),
        repo_count: stats_req.repo_count.unwrap_or(existing_stats.repo_count),
        followers: stats_req.followers.unwrap_or(existing_stats.followers),
        contributions: stats_req.contributions.unwrap_or(existing_stats.contributions),
        top_languages: stats_req.top_languages.clone().unwrap_or_else(|| existing_stats.top_languages.clone()),
        recent_activity: stats_req.recent_activity.clone().unwrap_or_else(|| existing_stats.recent_activity.clone()),
    };
    
    // Save updated stats
    let result = repo.update(&stats_id, updated_stats.clone()).await
        .map_err(|e| {
            error!("Failed to update GitHub stats: {}", e);
            AppError::internal_error(format!("Failed to update GitHub stats: {}", e))
        })?;
    
    info!("Updated GitHub stats for user: {}", result.username);
    
    Ok(HttpResponse::Ok().json(result))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_github_stats)
       .service(update_github_stats)
       .service(refresh_github_stats); // Add the refresh endpoint
}
