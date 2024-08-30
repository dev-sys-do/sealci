use sqlx::PgPool;
use std::sync::Arc;

use crate::parser::pipe_parser::Type;

pub struct Action {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    container_uri: String,
    r#type: Type,
    status: String,
}

pub struct ActionRepository {
    pool: Arc<PgPool>,
}

impl ActionRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, pipeline_id: i64, name: &String, container_uri: &String, r#type: &Type, status: &String) {
        // create a nex action in psql
        let row = sqlx::query_as!(
            Action,
            r#"INSERT INTO actions (pipeline_id, name, container_uri, type, status) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            pipeline_id,
            name,
            container_uri,
            &r#type.to_string(),
            status
        )
        .fetch_one(self.pool.as_ref())
        .await
        .unwrap();
    }
}
