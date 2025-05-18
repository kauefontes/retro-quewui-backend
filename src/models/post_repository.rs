use anyhow::Result;
use sqlx::{Pool, Sqlite, FromRow};
use serde::{Serialize, Deserialize};

use crate::models::post::Post;
use crate::models::repository::{Repository, vec_to_json, json_to_vec};

pub struct PostRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct PostRow {
    id: String,
    title: String,
    date: String,
    tags: String,
    excerpt: String,
    content: String,
}

impl Repository<Post> for PostRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn find_all(&self) -> Result<Vec<Post>> {
        let query = "
            SELECT id, title, date, tags, excerpt, content
            FROM posts
            ORDER BY date DESC
        ";
        
        let rows = sqlx::query_as::<_, PostRow>(query)
            .fetch_all(&self.pool)
            .await?;

        let posts = rows
            .into_iter()
            .map(|row| {
                let tags: Vec<String> = json_to_vec(&row.tags).unwrap_or_default();

                Post {
                    id: row.id,
                    title: row.title,
                    date: row.date,
                    tags,
                    excerpt: row.excerpt,
                    content: row.content,
                }
            })
            .collect();

        Ok(posts)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Post>> {
        let query = "
            SELECT id, title, date, tags, excerpt, content
            FROM posts
            WHERE id = ?
        ";
        
        let row = sqlx::query_as::<_, PostRow>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let post = match row {
            Some(row) => {
                let tags: Vec<String> = json_to_vec(&row.tags).unwrap_or_default();

                Some(Post {
                    id: row.id,
                    title: row.title,
                    date: row.date,
                    tags,
                    excerpt: row.excerpt,
                    content: row.content,
                })
            }
            None => None,
        };

        Ok(post)
    }

    async fn create(&self, item: Post) -> Result<Post> {
        let id = item.id.clone();
        let tags = vec_to_json(&item.tags)?;

        let query = "
            INSERT INTO posts (id, title, date, tags, excerpt, content)
            VALUES (?, ?, ?, ?, ?, ?)
        ";
        
        sqlx::query(query)
            .bind(id)
            .bind(&item.title)
            .bind(&item.date)
            .bind(tags)
            .bind(&item.excerpt)
            .bind(&item.content)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn update(&self, id: &str, item: Post) -> Result<Post> {
        let tags = vec_to_json(&item.tags)?;

        let query = "
            UPDATE posts
            SET title = ?, date = ?, tags = ?, excerpt = ?, content = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        sqlx::query(query)
            .bind(&item.title)
            .bind(&item.date)
            .bind(tags)
            .bind(&item.excerpt)
            .bind(&item.content)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let query = "
            DELETE FROM posts
            WHERE id = ?
        ";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
