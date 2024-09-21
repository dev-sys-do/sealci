use std::sync::Arc;

use sqlx::PgPool;

pub struct Postgres {
    pub pool: Arc<PgPool>,
}

impl Postgres {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url).await.unwrap();
        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}
