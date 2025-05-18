use anyhow::Result;
use sqlx::{Pool, Sqlite, FromRow};
use serde::{Serialize, Deserialize};

use crate::models::skill::Skill;
use crate::models::repository::{Repository, vec_to_json, json_to_vec};

pub struct SkillRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct SkillRow {
    id: String,
    category: String,
    items: String,
}

impl Repository<Skill> for SkillRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn find_all(&self) -> Result<Vec<Skill>> {
        let query = "
            SELECT id, category, items
            FROM skills
            ORDER BY category ASC
        ";
        
        let rows = sqlx::query_as::<_, SkillRow>(query)
            .fetch_all(&self.pool)
            .await?;

        let skills = rows
            .into_iter()
            .map(|row| {
                let items: Vec<String> = json_to_vec(&row.items).unwrap_or_default();

                Skill {
                    category: row.category,
                    items,
                }
            })
            .collect();

        Ok(skills)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Skill>> {
        let query = "
            SELECT id, category, items
            FROM skills
            WHERE id = ?
        ";
        
        let row = sqlx::query_as::<_, SkillRow>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        let skill = match row {
            Some(row) => {
                let items: Vec<String> = json_to_vec(&row.items).unwrap_or_default();

                Some(Skill {
                    category: row.category,
                    items,
                })
            }
            None => None,
        };

        Ok(skill)
    }

    async fn create(&self, item: Skill) -> Result<Skill> {
        let id = crate::models::repository::generate_id();
        let items = vec_to_json(&item.items)?;

        let query = "
            INSERT INTO skills (id, category, items)
            VALUES (?, ?, ?)
        ";
        
        sqlx::query(query)
            .bind(&id)
            .bind(&item.category)
            .bind(items)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn update(&self, id: &str, item: Skill) -> Result<Skill> {
        let items = vec_to_json(&item.items)?;

        let query = "
            UPDATE skills
            SET category = ?, items = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        ";
        
        sqlx::query(query)
            .bind(&item.category)
            .bind(items)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(item)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let query = "
            DELETE FROM skills
            WHERE id = ?
        ";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
