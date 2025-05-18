use anyhow::Result;
use sqlx::{Pool, Sqlite, FromRow};
use serde::{Serialize, Deserialize};

use crate::models::github_stats::{GithubStats, TopLanguage, RecentActivity};
use crate::models::repository::Repository;

pub struct GithubStatsRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct GithubStatsRow {
    id: String,
    username: String,
    repo_count: i32,
    followers: i32,
    contributions: i32,
    top_languages: String,
    recent_activity: String,
}

impl Repository<GithubStats> for GithubStatsRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn find_all(&self) -> Result<Vec<GithubStats>> {
        // For github stats, we typically have only one record, but we'll follow the
        // Repository pattern for consistency
        let query = "
            SELECT id, username, repo_count, followers, contributions, top_languages, recent_activity
            FROM github_stats
            LIMIT 1
        ";
        
        let rows = sqlx::query_as::<_, GithubStatsRow>(query)
            .fetch_all(&self.pool)
            .await?;

        let stats = rows
            .into_iter()
            .map(|row| {
                let top_languages: Vec<TopLanguage> = serde_json::from_str(&row.top_languages).unwrap_or_default();
                let recent_activity: Vec<RecentActivity> = serde_json::from_str(&row.recent_activity).unwrap_or_default();

                GithubStats {
                    username: row.username,
                    repo_count: row.repo_count,
                    followers: row.followers,
                    contributions: row.contributions,
                    top_languages,
                    recent_activity,
                }
            })
            .collect();

        Ok(stats)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<GithubStats>> {
        let query = "
            SELECT id, username, repo_count, followers, contributions, top_languages, recent_activity
            FROM github_stats
            WHERE id = ?
        ";
        
        let row = sqlx::query_as::<_, GithubStatsRow>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let stats = match row {
            Some(row) => {
                let top_languages: Vec<TopLanguage> = serde_json::from_str(&row.top_languages).unwrap_or_default();
                let recent_activity: Vec<RecentActivity> = serde_json::from_str(&row.recent_activity).unwrap_or_default();

                Some(GithubStats {
                    username: row.username,
                    repo_count: row.repo_count,
                    followers: row.followers,
                    contributions: row.contributions,
                    top_languages,
                    recent_activity,
                })
            }
            None => None,
        };

        Ok(stats)
    }

    async fn create(&self, item: GithubStats) -> Result<GithubStats> {
        let id = crate::models::repository::generate_id();
        let top_languages = serde_json::to_string(&item.top_languages)?;
        let recent_activity = serde_json::to_string(&item.recent_activity)?;

        let query = "
            INSERT INTO github_stats (id, username, repo_count, followers, contributions, top_languages, recent_activity)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ";
        
        sqlx::query(query)
            .bind(&id)
            .bind(&item.username)
            .bind(item.repo_count)
            .bind(item.followers)
            .bind(item.contributions)
            .bind(top_languages)
            .bind(recent_activity)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn update(&self, id: &str, item: GithubStats) -> Result<GithubStats> {
        let top_languages = serde_json::to_string(&item.top_languages)?;
        let recent_activity = serde_json::to_string(&item.recent_activity)?;

        let query = "
            UPDATE github_stats
            SET username = ?, repo_count = ?, followers = ?, contributions = ?, 
                top_languages = ?, recent_activity = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        sqlx::query(query)
            .bind(&item.username)
            .bind(item.repo_count)
            .bind(item.followers)
            .bind(item.contributions)
            .bind(top_languages)
            .bind(recent_activity)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let query = "
            DELETE FROM github_stats
            WHERE id = ?
        ";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
