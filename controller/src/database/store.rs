use sqlx::PgPool;
use sqlx::FromRow;
use crate::Database;

pub struct Store {
    pub pool: PgPool,
}

#[derive(Debug, FromRow)]
pub struct Pipeline {
    pub id: i32,
    pub repository_url: String,
}

#[derive(Debug, FromRow)]
pub struct Action {
    pub id: i32,
    pub pipeline_id: i64,
    pub name: String,
    pub logs: String,
    pub status: String,
    pub action_type: String,
    pub container_uri: String,
    pub commands: sqlx::types::Json<Vec<String>>,
    pub created_at: String,
}


impl Store {
    pub fn new(database: &Database) -> Self {
        Store {
            pool: database.pool.clone(),
        }
    }

    // Example: Adding a method to fetch something from the database
    pub async fn get_pipelines(&self) -> Result<Vec<Pipeline>, sqlx::Error> {
        let rows = sqlx::query_as!(
            Pipeline,
            r#"
            SELECT * FROM pipelines
            "#)
            .fetch_all(&self.pool)
            .await?;
    
        Ok(rows)
    }

    pub async fn get_pipeline(&self, id: i32) -> Result<Pipeline, sqlx::Error> {
        let row = sqlx::query_as!(
            Pipeline,
            r#"
            SELECT * FROM pipelines WHERE id = $1
            "#,
            id)
            .fetch_one(&self.pool)
            .await?;
    
        Ok(row)
    }

    pub async fn create_pipeline(&self, repository_url: &str) -> Result<Pipeline, sqlx::Error> {
        let row = sqlx::query_as!(
            Pipeline,
            r#"
            INSERT INTO pipelines (repository_url)
            VALUES ($1)
            RETURNING *
            "#,
            repository_url)
            .fetch_one(&self.pool)
            .await?;
    
        Ok(row)
    }
}
