use async_trait::async_trait;

use crate::domain::entities::scheduler::SchedulerError;

#[async_trait]
pub trait SchedulerService: Send + Sync {
    // async fn send_action(
    //     &self,
    //     action: Action,
    //     repo_url: String,
    // ) -> Result<(), Box<dyn std::error::Error>>;
    async fn execute_pipeline(&self, pipeline_id: i64) -> Result<(), SchedulerError>;
}
