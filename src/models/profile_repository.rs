use anyhow::Result;
use sqlx::{Pool, Sqlite, FromRow};
use serde::{Serialize, Deserialize};

use crate::models::profile::{Profile, SocialLink, Education, Language};
use crate::models::repository::{Repository, vec_to_json, json_to_vec};

pub struct ProfileRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct ProfileRow {
    id: String,
    bio: String,
    social_links: String,
    education: String,
    languages: String,
}

impl Repository<Profile> for ProfileRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn find_all(&self) -> Result<Vec<Profile>> {
        // For profile, we typically have only one record, but we'll follow the
        // Repository pattern for consistency
        let query = "
            SELECT id, bio, social_links, education, languages
            FROM profiles
            LIMIT 1
        ";
        
        let rows = sqlx::query_as::<_, ProfileRow>(query)
            .fetch_all(&self.pool)
            .await?;

        let profiles = rows
            .into_iter()
            .map(|row| {
                let bio: Vec<String> = json_to_vec(&row.bio).unwrap_or_default();
                let social_links: Vec<SocialLink> = serde_json::from_str(&row.social_links).unwrap_or_default();
                let education: Vec<Education> = serde_json::from_str(&row.education).unwrap_or_default();
                let languages: Vec<Language> = serde_json::from_str(&row.languages).unwrap_or_default();

                Profile {
                    bio,
                    social_links,
                    education,
                    languages,
                }
            })
            .collect();

        Ok(profiles)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Profile>> {
        let query = "
            SELECT id, bio, social_links, education, languages
            FROM profiles
            WHERE id = ?
        ";
        
        let row = sqlx::query_as::<_, ProfileRow>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let profile = match row {
            Some(row) => {
                let bio: Vec<String> = json_to_vec(&row.bio).unwrap_or_default();
                let social_links: Vec<SocialLink> = serde_json::from_str(&row.social_links).unwrap_or_default();
                let education: Vec<Education> = serde_json::from_str(&row.education).unwrap_or_default();
                let languages: Vec<Language> = serde_json::from_str(&row.languages).unwrap_or_default();

                Some(Profile {
                    bio,
                    social_links,
                    education,
                    languages,
                })
            }
            None => None,
        };

        Ok(profile)
    }

    async fn create(&self, item: Profile) -> Result<Profile> {
        let id = crate::models::repository::generate_id();
        let bio = vec_to_json(&item.bio)?;
        let social_links = serde_json::to_string(&item.social_links)?;
        let education = serde_json::to_string(&item.education)?;
        let languages = serde_json::to_string(&item.languages)?;

        let query = "
            INSERT INTO profiles (id, bio, social_links, education, languages)
            VALUES (?, ?, ?, ?, ?)
        ";
        
        sqlx::query(query)
            .bind(id)
            .bind(bio)
            .bind(social_links)
            .bind(education)
            .bind(languages)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn update(&self, id: &str, item: Profile) -> Result<Profile> {
        log::info!("Updating profile with ID: {}", id);
        
        let bio = vec_to_json(&item.bio)?;
        let social_links = serde_json::to_string(&item.social_links)?;
        let education = serde_json::to_string(&item.education)?;
        let languages = serde_json::to_string(&item.languages)?;
        
        log::info!("Bio JSON: {}", bio);
        log::info!("Social links JSON: {}", social_links);
        log::info!("Education JSON: {}", education);
        log::info!("Languages JSON: {}", languages);

        let query = "
            UPDATE profiles
            SET bio = ?, social_links = ?, education = ?, languages = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        let result = sqlx::query(query)
            .bind(bio)
            .bind(social_links)
            .bind(education)
            .bind(languages)
            .bind(id)
            .execute(&self.pool)
            .await?;
            
        log::info!("Update affected {} rows", result.rows_affected());

        Ok(item)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let query = "
            DELETE FROM profiles
            WHERE id = ?
        ";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
