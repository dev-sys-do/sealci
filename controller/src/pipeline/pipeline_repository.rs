use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDTO {
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

    pub async fn find_all(&self) -> Result<Vec<PipelineDTO>, sqlx::Error> {
        let rows = sqlx::query_as!(PipelineDTO, r#"SELECT * FROM pipelines"#)
            .fetch_all(&*self.pool)
            .await?;

        Ok(rows)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<PipelineDTO, sqlx::Error> {
        let row = sqlx::query_as!(PipelineDTO, r#"SELECT * FROM pipelines WHERE id = $1"#, id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(row)
    }

    pub async fn create(&self, repository_url: &String) -> Result<PipelineDTO, sqlx::Error> {
        let row = sqlx::query_as!(
            PipelineDTO,
            r#"INSERT INTO pipelines (repository_url) VALUES ($1) RETURNING *"#,
            repository_url
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(row)
    }
}
