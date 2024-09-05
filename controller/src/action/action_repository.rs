use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    action::action_service::{ActionDTO, CommandDTO},
    parser::pipe_parser::Type,
};

#[derive(Debug, Clone)]
pub struct Action {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub container_uri: String,
    pub commands: Vec<String>,
    r#type: Type,
    status: String,
}

pub struct Command {
    command: String,
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
        commands: Vec<String>,
    ) -> Result<Action, sqlx::Error> {
        // create a nex action in psql
        let action = sqlx::query_as!(
            ActionDTO,
            r#"INSERT INTO actions (pipeline_id, name, container_uri, type, status) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            pipeline_id,
            name,
            container_uri,
            &r#type.to_string(),
            status
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        for command in &commands {
            sqlx::query_as!(
                CommandDTO,
                r#"INSERT INTO commands (action_id, command) VALUES ($1, $2) RETURNING *"#,
                action.id,
                command
            )
            .fetch_one(self.pool.as_ref())
            .await?;
        }

        Ok(Action {
            id: action.id.unwrap(), //we can because we just fetched it from database
            pipeline_id: action.pipeline_id,
            name: action.name,
            container_uri: action.container_uri,
            r#type: action.r#type,
            status: action.status,
            commands,
        })
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Action, sqlx::Error> {
        let action = sqlx::query_as!(ActionDTO, r#"SELECT * FROM actions WHERE id = $1"#, id)
            .fetch_one(&*self.pool)
            .await?;

        let commands = self.get_commands(action.id.unwrap()).await?;

        Ok(Action {
            id: action.id.unwrap(),
            pipeline_id: action.pipeline_id,
            name: action.name,
            container_uri: action.container_uri,
            r#type: action.r#type,
            status: action.status,
            commands,
        })
    }

    pub async fn find_by_pipeline_id(&self, pipeline_id: i64) -> Result<Vec<Action>, sqlx::Error> {
        let mut actions = Vec::new();
        let actions_dto = sqlx::query_as!(
            ActionDTO,
            r#"SELECT * FROM actions WHERE pipeline_id = $1"#,
            pipeline_id
        )
        .fetch_all(&*self.pool)
        .await?;

        for action in actions_dto {
            let commands = self.get_commands(action.id.unwrap()).await?;
            actions.push(Action {
                id: action.id.unwrap(),
                pipeline_id: action.pipeline_id,
                name: action.name,
                container_uri: action.container_uri,
                r#type: action.r#type,
                status: action.status,
                commands,
            });
        }

        Ok(actions)
    }

    pub async fn get_commands(&self, action_id: i64) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query_as!(
            Command,
            r#"SELECT command FROM commands WHERE action_id=$1 ORDER BY id"#,
            action_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows.iter().map(|row| row.command.clone()).collect())
    }
}
