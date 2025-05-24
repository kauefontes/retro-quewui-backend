use anyhow::Result;
use sqlx::{Pool, Sqlite, FromRow};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::models::github_profile::{GitHubProfile, GitHubOrganization, GitHubRepository, GitHubActivityItem};
use crate::services::github_service::GitHubService;

pub struct GitHubProfileRepository {
    pool: Pool<Sqlite>,
    github_service: GitHubService,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct GitHubProfileRow {
    id: String,
    data: String,  // Stores the entire GitHub profile as JSON
    last_updated: String,
}

impl GitHubProfileRepository {
    pub fn new(pool: Pool<Sqlite>, github_service: GitHubService) -> Self {
        Self { pool, github_service }
    }
    
    pub async fn get_github_profile(&self) -> Result<GitHubProfile> {
        // First check if we have a cached profile and it's recent (less than 1 hour old)
        if let Some(cached_profile) = self.get_cached_profile().await? {
            return Ok(cached_profile);
        }
        
        // If not, fetch fresh data from GitHub
        let user = self.github_service.get_user_profile().await?;
        let orgs = self.github_service.get_user_organizations().await?;
        let repos = self.github_service.get_user_repos(10, 1).await?;
        let activities = self.github_service.get_user_activity(20).await?;
        
        // Transform the data into our GitHubProfile model
        let organizations = orgs.into_iter()
            .map(|org| {
                let login_clone = org.login.clone(); // Clone before it's moved
                GitHubOrganization {
                    login: org.login,
                    id: org.id,
                    avatar_url: org.avatar_url,
                    description: org.description,
                    html_url: format!("https://github.com/{}", login_clone),
                }
            })
            .collect();
            
        let top_repositories = repos.into_iter()
            .map(|repo| GitHubRepository {
                name: repo.name,
                full_name: repo.full_name,
                html_url: repo.html_url,
                description: repo.description,
                language: repo.language,
                stargazers_count: repo.stargazers_count,
                forks_count: repo.forks_count,
                topics: repo.topics.unwrap_or_default(),
                updated_at: repo.updated_at,
            })
            .collect();
            
        let recent_activity = activities.into_iter()
            .map(|activity| {
                let repo_name = activity.repo.name.clone();
                GitHubActivityItem {
                    event_type: activity.r#type,
                    repo_name,
                    repo_url: format!("https://github.com/{}", activity.repo.name),
                    created_at: activity.created_at,
                    details: activity.payload,
                }
            })
            .collect();
            
        let profile = GitHubProfile {
            username: user.login.clone(),
            display_name: user.name.unwrap_or_else(|| user.login.clone()),
            avatar_url: user.avatar_url,
            bio: user.bio,
            location: user.location,
            blog: user.blog,
            twitter_username: user.twitter_username,
            company: user.company,
            followers: user.followers,
            following: user.following,
            public_repos: user.public_repos,
            public_gists: user.public_gists,
            html_url: user.html_url,
            created_at: user.created_at,
            organizations,
            top_repositories,
            recent_activity,
        };
        
        // Cache the profile
        self.cache_profile(&profile).await?;
        
        Ok(profile)
    }
    
    async fn get_cached_profile(&self) -> Result<Option<GitHubProfile>> {
        let query = "
            SELECT id, data, last_updated 
            FROM github_profiles 
            ORDER BY last_updated DESC 
            LIMIT 1
        ";
        
        let row = sqlx::query_as::<_, GitHubProfileRow>(query)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = row {
            // Check if the cached data is less than 1 hour old
            let last_updated = DateTime::parse_from_rfc3339(&row.last_updated)?;
            let now = Utc::now();
            // Convert last_updated to UTC for comparison
            let last_updated_utc = last_updated.with_timezone(&Utc);
            let age = now.signed_duration_since(last_updated_utc);
            
            if age.num_minutes() < 60 {
                let profile: GitHubProfile = serde_json::from_str(&row.data)?;
                return Ok(Some(profile));
            }
        }
        
        Ok(None)
    }
    
    async fn cache_profile(&self, profile: &GitHubProfile) -> Result<()> {
        let id = crate::models::repository::generate_id();
        let data = serde_json::to_string(profile)?;
        let now = Utc::now().to_rfc3339();
        
        // First try to update any existing profile
        let update_query = "
            UPDATE github_profiles
            SET data = ?, last_updated = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = (SELECT id FROM github_profiles ORDER BY last_updated DESC LIMIT 1)
        ";
        
        let result = sqlx::query(update_query)
            .bind(&data)
            .bind(&now)
            .execute(&self.pool)
            .await?;
            
        // If no rows were updated, insert a new row
        if result.rows_affected() == 0 {
            let insert_query = "
                INSERT INTO github_profiles (id, data, last_updated)
                VALUES (?, ?, ?)
            ";
            
            sqlx::query(insert_query)
                .bind(id)
                .bind(data)
                .bind(now)
                .execute(&self.pool)
                .await?;
        }
            
        Ok(())
    }
}
