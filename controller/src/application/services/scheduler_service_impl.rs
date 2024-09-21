use crate::{
    application::ports::{action_service::ActionService, scheduler_service::SchedulerService},
    domain::{entities::scheduler::SchedulerError, services::scheduler_client::SchedulerClient},
};
use async_trait::async_trait;
use futures::lock::Mutex;
use tracing::info;
use std::sync::Arc;

pub struct SchedulerServiceImpl {
    action_service: Arc<Box<dyn ActionService + Send + Sync>>,
    scheduler_client: Arc<Mutex<Box<dyn SchedulerClient + Send + Sync>>>
}

impl SchedulerServiceImpl {
    pub fn new(action_service: Arc<Box<dyn ActionService + Send + Sync>>, scheduler_client: Arc<Mutex<Box<dyn SchedulerClient + Send + Sync>>>) -> Self {
        Self { action_service, scheduler_client }
    }
}

#[async_trait]
impl SchedulerService for SchedulerServiceImpl {
    async fn execute_pipeline(&self, pipeline_id: i64) -> Result<(), SchedulerError> {
        let mut actions = self
            .action_service
            .find_by_pipeline_id(pipeline_id)
            .await.unwrap();

        
        actions.sort_by_key(|action| action.id);

        
        let mut scheduler_client = self.scheduler_client.lock().await;

        for action in actions {

          info!("Scheduling action: {:?}", action);
        }
        //let t = scheduler_client.schedule_action(action_request).await;
        Ok(())
    }
}
