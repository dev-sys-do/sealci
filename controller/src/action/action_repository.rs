use sqlx::PgPool;
use std::sync::Arc;

pub struct Action {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    container_uri: String,
    r#type: String,
    status: String,
}

pub struct ActionRepository {
    pool: Arc<PgPool>,
}

impl ActionRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, pipeline_id: i64, name: String) {
        // create a nex action in psql
        let row = sqlx::query_as!(
            Action,
            r#"INSERT INTO actions (pipeline_id, name, container_uri, type, status) VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            pipeline_id,
            name,
            "amazon/aws-cli",
            "container",
            "pending"
        )
        .fetch_one(self.pool.as_ref())
        .await
        .unwrap();
    }
}
