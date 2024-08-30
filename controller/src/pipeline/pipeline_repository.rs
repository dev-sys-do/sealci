use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: i64,
    pub repository_url: String,
}
pub struct PipelineRepository {
    pool: Arc<PgPool>,
}

impl PipelineRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Pipeline>, sqlx::Error> {
        let rows = sqlx::query_as!(Pipeline, r#"SELECT * FROM pipelines"#)
            .fetch_all(&*self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Pipeline, sqlx::Error> {
        let row = sqlx::query_as!(Pipeline, r#"SELECT * FROM pipelines WHERE id = $1"#, id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(row)
    }

    pub async fn create(&self, repository_url: &String) -> Result<Pipeline, sqlx::Error> {
        let row = sqlx::query_as!(
            Pipeline,
            r#"INSERT INTO pipelines (repository_url) VALUES ($1) RETURNING *"#,
            repository_url
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(row)
    }
}
