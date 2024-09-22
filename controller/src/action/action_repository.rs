use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    action::action_service::ActionDTO, grpc_scheduler::ActionStatus, parser::pipe_parser::Type,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub container_uri: String,
    pub commands: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    r#type: Type,
    status: String,
}

#[derive(Debug)]
pub enum ActionCreationError {
    UnknownStatus,
}

impl Action {
    pub fn new(
        id: i64,
        pipeline_id: i64,
        name: String,
        container_uri: String,
        commands: Vec<String>,
        r#type: Type,
        status: String,
    ) -> Result<Self, ActionCreationError> {
        let status = ActionStatus::from_str_name(status.as_str());
        if status.is_none() {
            return Err(ActionCreationError::UnknownStatus);
        }
        let status = ActionStatus::as_str_name(&status.unwrap()).to_string();
        return Ok(Action {
            id,
            pipeline_id,
            name,
            container_uri,
            status,
            logs: None,
            r#type,
            commands,
        });
    }
}

pub struct ActionRepository {
    pool: Arc<PgPool>,
}

impl ActionRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        pipeline_id: i64,
        name: &String,
        container_uri: &String,
        r#type: &Type,
        status: &String,
    ) -> Result<ActionDTO, sqlx::Error> {
        // create a nex action in psql
        sqlx::query_as!(
            ActionDTO,
            r#"INSERT INTO actions (pipeline_id, name, container_uri, type, status) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            pipeline_id,
            name,
            container_uri,
            &r#type.to_string(),
            status
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    #[allow(dead_code)]
    pub async fn find_by_id(&self, id: i64) -> Result<ActionDTO, sqlx::Error> {
        sqlx::query_as!(
            ActionDTO,
            r#"SELECT * FROM actions WHERE id = $1 ORDER BY id"#,
            id
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn alter_status(&self, status: &str, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE actions SET status = $1 WHERE id = $2"#,
            status,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn find_by_pipeline_id(
        &self,
        pipeline_id: i64,
    ) -> Result<Vec<ActionDTO>, sqlx::Error> {
        sqlx::query_as!(
            ActionDTO,
            r#"SELECT * FROM actions WHERE pipeline_id = $1 ORDER BY id"#,
            pipeline_id
        )
        .fetch_all(&*self.pool)
        .await
    }
}
