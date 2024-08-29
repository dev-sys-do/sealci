use crate::database::store::Pipeline;
use sqlx::{query, PgPool};
use std::sync::Arc;

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

    pub async fn find_by_id(&self, id: i32) -> Result<Pipeline, sqlx::Error> {
        let row = sqlx::query_as!(Pipeline, r#"SELECT * FROM pipelines WHERE id = $1"#, id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(row)
    }

    pub async fn create(&self, repository_url: &str) -> Result<Pipeline, sqlx::Error> {
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
