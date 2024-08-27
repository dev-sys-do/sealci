use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(
        database_url: &String
    ) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create pool");

        Database { pool }
    }
}

