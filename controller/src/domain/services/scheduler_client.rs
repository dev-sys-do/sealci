use std::pin::Pin;

use futures::Stream;
use std::error::Error;
use tonic::async_trait;

use crate::domain::entities::action::{ActionRequest, ActionResponse};

#[async_trait]
pub trait SchedulerClient: Send + Sync {
    async fn schedule_action(
        &self,
        action_request: ActionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ActionResponse, Box<dyn Error + Send + Sync>>> + Send>>,
        Box<dyn Error + Send + Sync>,
    >;
}
