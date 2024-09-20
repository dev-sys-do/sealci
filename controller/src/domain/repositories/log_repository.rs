use async_trait::async_trait;

use crate::domain::entities::log::{Log, LogError};

#[async_trait]
pub trait LogRepository {
    async fn create(&self, action_id: i64, data: String) -> Result<Log, LogError>;
}
