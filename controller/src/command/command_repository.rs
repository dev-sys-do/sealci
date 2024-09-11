use std::sync::Arc;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct CommandDTO {
    pub id: i64,
    pub action_id: i64,
    pub command: String,
}

pub struct CommandRepository {
    pool: Arc<PgPool>,
}

impl CommandRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        action_id: i64,
        command: &String,
    ) -> Result<CommandDTO, sqlx::Error> {
        let command_row = sqlx::query_as!(
            CommandDTO,
            r#"INSERT INTO commands (action_id, command) VALUES ($1, $2) RETURNING *"#,
            action_id,
            command
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(CommandDTO {
            id: command_row.id,
            action_id: command_row.action_id,
            command: command_row.command,
        })
    }

    #[allow(dead_code)]
    pub async fn get_all(&self, action_id: i64) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query_as!(
            CommandDTO,
            r#"SELECT id, action_id, command FROM commands WHERE action_id=$1 ORDER BY id"#,
            action_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(rows.iter().map(|row| row.command.clone()).collect())
    }
}
