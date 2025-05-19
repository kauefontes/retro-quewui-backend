use anyhow::Result;
use sqlx::{Pool, Sqlite, FromRow};
use serde::{Serialize, Deserialize};

use crate::models::project::Project;
use crate::models::repository::{Repository, vec_to_json, json_to_vec};

pub struct ProjectRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct ProjectRow {
    id: String,
    title: String,
    description: String,
    technologies: String,
    github_url: Option<String>,
    live_url: Option<String>,
    image_url: Option<String>,
    image_urls: Option<String>,
    year: i32,
    highlights: String,
}

impl Repository<Project> for ProjectRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn find_all(&self) -> Result<Vec<Project>> {
        let query = "
            SELECT id, title, description, technologies, github_url, live_url, image_url, image_urls, year, highlights
            FROM projects
            ORDER BY year DESC
        ";
        
        let rows = sqlx::query_as::<_, ProjectRow>(query)
            .fetch_all(&self.pool)
            .await?;

        let projects = rows
            .into_iter()
            .map(|row| {
                let technologies: Vec<String> = json_to_vec(&row.technologies).unwrap_or_default();
                let highlights: Vec<String> = json_to_vec(&row.highlights).unwrap_or_default();
                let image_urls: Option<Vec<String>> = row.image_urls
                    .as_ref()
                    .and_then(|urls| json_to_vec(urls).ok());

                Project {
                    id: row.id,
                    title: row.title,
                    description: row.description,
                    technologies,
                    github_url: row.github_url,
                    live_url: row.live_url,
                    image_url: row.image_url,
                    image_urls,
                    year: row.year,
                    highlights,
                }
            })
            .collect();

        Ok(projects)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Project>> {
        let query = "
            SELECT id, title, description, technologies, github_url, live_url, image_url, image_urls, year, highlights
            FROM projects
            WHERE id = ?
        ";
        
        let row = sqlx::query_as::<_, ProjectRow>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let project = match row {
            Some(row) => {
                let technologies: Vec<String> = json_to_vec(&row.technologies).unwrap_or_default();
                let highlights: Vec<String> = json_to_vec(&row.highlights).unwrap_or_default();
                let image_urls: Option<Vec<String>> = row.image_urls
                    .as_ref()
                    .and_then(|urls| json_to_vec(urls).ok());

                Some(Project {
                    id: row.id,
                    title: row.title,
                    description: row.description,
                    technologies,
                    github_url: row.github_url,
                    live_url: row.live_url,
                    image_url: row.image_url,
                    image_urls,
                    year: row.year,
                    highlights,
                })
            }
            None => None,
        };

        Ok(project)
    }

    async fn create(&self, item: Project) -> Result<Project> {
        let id = item.id.clone();
        let technologies = vec_to_json(&item.technologies)?;
        let highlights = vec_to_json(&item.highlights)?;
        let image_urls = match &item.image_urls {
            Some(urls) => Some(vec_to_json(urls)?),
            None => None,
        };

        let query = "
            INSERT INTO projects (id, title, description, technologies, github_url, live_url, image_url, image_urls, year, highlights)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";
        
        sqlx::query(query)
            .bind(id)
            .bind(&item.title)
            .bind(&item.description)
            .bind(technologies)
            .bind(&item.github_url)
            .bind(&item.live_url)
            .bind(&item.image_url)
            .bind(image_urls)
            .bind(item.year)
            .bind(highlights)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn update(&self, id: &str, item: Project) -> Result<Project> {
        let technologies = vec_to_json(&item.technologies)?;
        let highlights = vec_to_json(&item.highlights)?;
        let image_urls = match &item.image_urls {
            Some(urls) => Some(vec_to_json(urls)?),
            None => None,
        };

        let query = "
            UPDATE projects
            SET title = ?, description = ?, technologies = ?, github_url = ?, live_url = ?, image_url = ?, image_urls = ?, year = ?, highlights = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        sqlx::query(query)
            .bind(&item.title)
            .bind(&item.description)
            .bind(technologies)
            .bind(&item.github_url)
            .bind(&item.live_url)
            .bind(&item.image_url)
            .bind(image_urls)
            .bind(item.year)
            .bind(highlights)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let query = "
            DELETE FROM projects
            WHERE id = ?
        ";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

// No seeding functions - using real database data instead
