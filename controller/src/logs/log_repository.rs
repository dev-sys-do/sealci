use std::sync::Arc;

use sqlx::PgPool;

use super::Log;

#[derive(Debug, Clone)]
pub struct LogDTO {
    #[allow(dead_code)]
    pub id: Option<i64>,
    #[allow(dead_code)]
    pub action_id: i64,
    pub data: String,
}

pub struct LogRepository {
    pool: Arc<PgPool>,
}

impl LogRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, action_id: i64, data: &String) -> Result<Log, sqlx::Error> {
        let log_row = sqlx::query_as!(
            LogDTO,
            r#"INSERT INTO logs (action_id, data) VALUES ($1, $2) RETURNING *"#,
            action_id,
            data
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(Log {
            message: log_row.data,
        })
    }

    pub async fn find_by_action_id(&self, action_id: i64) -> Result<Vec<Log>, sqlx::Error> {
        let logs = sqlx::query_as!(
            LogDTO,
            r#"SELECT * FROM logs WHERE action_id = $1"#,
            action_id
        )
        .fetch_all(self.pool.as_ref())
        .await?;

        Ok(logs.into_iter().map(|log| Log { message: log.data }).collect())
    }
}
