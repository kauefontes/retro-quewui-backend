use anyhow::Result;
use sqlx::{Pool, Sqlite, FromRow};
use serde::{Serialize, Deserialize};

use crate::models::experience::Experience;
use crate::models::repository::{Repository, vec_to_json, json_to_vec};

pub struct ExperienceRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct ExperienceRow {
    id: String,
    company: String,
    position: String,
    start_date: String,
    end_date: Option<String>,
    description: String,
    technologies: String,
    highlights: String,
}

impl Repository<Experience> for ExperienceRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn find_all(&self) -> Result<Vec<Experience>> {
        let query = "
            SELECT id, company, position, start_date, end_date, description, technologies, highlights
            FROM experiences
            ORDER BY start_date DESC
        ";
        
        let rows = sqlx::query_as::<_, ExperienceRow>(query)
            .fetch_all(&self.pool)
            .await?;

        let experiences = rows
            .into_iter()
            .map(|row| {
                let technologies: Vec<String> = json_to_vec(&row.technologies).unwrap_or_default();
                let highlights: Vec<String> = json_to_vec(&row.highlights).unwrap_or_default();

                Experience {
                    id: row.id,
                    company: row.company,
                    position: row.position,
                    start_date: row.start_date,
                    end_date: row.end_date,
                    description: row.description,
                    technologies,
                    highlights,
                }
            })
            .collect();

        Ok(experiences)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Experience>> {
        let query = "
            SELECT id, company, position, start_date, end_date, description, technologies, highlights
            FROM experiences
            WHERE id = ?
        ";
        
        let row = sqlx::query_as::<_, ExperienceRow>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let experience = match row {
            Some(row) => {
                let technologies: Vec<String> = json_to_vec(&row.technologies).unwrap_or_default();
                let highlights: Vec<String> = json_to_vec(&row.highlights).unwrap_or_default();

                Some(Experience {
                    id: row.id,
                    company: row.company,
                    position: row.position,
                    start_date: row.start_date,
                    end_date: row.end_date,
                    description: row.description,
                    technologies,
                    highlights,
                })
            }
            None => None,
        };

        Ok(experience)
    }

    async fn create(&self, item: Experience) -> Result<Experience> {
        let id = item.id.clone();
        let technologies = vec_to_json(&item.technologies)?;
        let highlights = vec_to_json(&item.highlights)?;

        let query = "
            INSERT INTO experiences (id, company, position, start_date, end_date, description, technologies, highlights)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ";
        
        sqlx::query(query)
            .bind(id)
            .bind(&item.company)
            .bind(&item.position)
            .bind(&item.start_date)
            .bind(&item.end_date)
            .bind(&item.description)
            .bind(technologies)
            .bind(highlights)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn update(&self, id: &str, item: Experience) -> Result<Experience> {
        let technologies = vec_to_json(&item.technologies)?;
        let highlights = vec_to_json(&item.highlights)?;

        let query = "
            UPDATE experiences
            SET company = ?, position = ?, start_date = ?, end_date = ?, description = ?, technologies = ?, highlights = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        sqlx::query(query)
            .bind(&item.company)
            .bind(&item.position)
            .bind(&item.start_date)
            .bind(&item.end_date)
            .bind(&item.description)
            .bind(technologies)
            .bind(highlights)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let query = "
            DELETE FROM experiences
            WHERE id = ?
        ";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
