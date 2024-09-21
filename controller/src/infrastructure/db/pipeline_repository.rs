use crate::domain::entities::pipeline::{Pipeline, PipelineError};
use crate::domain::repositories::pipeline_repository::PipelineRepository;
use crate::infrastructure::db::postgres::Postgres;
use async_trait::async_trait;
use std::sync::Arc;

pub struct PostgresPipelineRepository {
    pub postgres: Arc<Postgres>,
}

impl PostgresPipelineRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

#[async_trait]
impl PipelineRepository for PostgresPipelineRepository {
    async fn create(
        &self,
        repository_url: String,
        name: String,
    ) -> Result<Pipeline, PipelineError> {
        let result = sqlx::query_as!(
            Pipeline,
            "INSERT INTO pipelines (repository_url, name) VALUES ($1, $2) RETURNING id, repository_url, name",
            repository_url, name
        )
        .fetch_one(&*self.postgres.get_pool())
        .await;

        result
            .map(|row| Pipeline {
                id: row.id,
                repository_url: row.repository_url,
                name: row.name,
            })
            .map_err(PipelineError::DatabaseError)
    }

    async fn find_all(&self) -> Result<Vec<Pipeline>, PipelineError> {
        let result = sqlx::query_as!(Pipeline, "SELECT id, repository_url, name FROM pipelines")
            .fetch_all(&*self.postgres.get_pool())
            .await;

        result.map_err(PipelineError::DatabaseError)
    }

    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError> {
        let result = sqlx::query_as!(
            Pipeline,
            "SELECT id, repository_url, name FROM pipelines WHERE id = $1",
            pipeline_id
        )
        .fetch_one(&*self.postgres.get_pool())
        .await;

        match result {
            Ok(pipeline) => Ok(pipeline),
            Err(sqlx::Error::RowNotFound) => Err(PipelineError::NotFound),
            Err(err) => Err(PipelineError::DatabaseError(err)),
        }
    }
}
