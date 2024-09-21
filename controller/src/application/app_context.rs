use std::sync::Arc;

use futures::lock::Mutex;

use crate::{
    domain::{repositories::{
        action_repository::ActionRepository, command_repository::CommandRepository,
        pipeline_repository::PipelineRepository,
    }, services::scheduler_client::SchedulerClient},
    infrastructure::{db::{
        action_repository::PostgresActionRepository, command_repository::PostgresCommandRepository,
        pipeline_repository::PostgresPipelineRepository, postgres::Postgres,
    }, grpc::grpc_scheduler_client::GrpcSchedulerClient},
};

use super::{ports::{
    action_service::ActionService, command_service::CommandService,
    pipeline_service::PipelineService, scheduler_service::SchedulerService,
}, services::{action_service::ActionServiceImpl, command_service::CommandServiceImpl, pipeline_service::PipelineServiceImpl, scheduler_service_impl::SchedulerServiceImpl}};

pub struct AppContext {
    pub pipeline_service: Arc<Box<dyn PipelineService + Send + Sync>>,
    pub action_service: Arc<Box<dyn ActionService + Send + Sync>>,
    pub command_service: Arc<Box<dyn CommandService + Send + Sync>>,
    pub scheduler_service: Arc<Box<dyn SchedulerService + Send + Sync>>,
}

impl AppContext {
    pub async fn initialize(postgres: Arc<Postgres>, grpc_url: &str) -> Self {
        let pipeline_repository: Arc<Box<dyn PipelineRepository + Send + Sync>> = Arc::new(
            Box::new(PostgresPipelineRepository::new(Arc::clone(&postgres))),
        );

        let action_repository: Arc<Box<dyn ActionRepository + Send + Sync>> = Arc::new(Box::new(
            PostgresActionRepository::new(Arc::clone(&postgres)),
        ));

        let command_repository: Arc<Box<dyn CommandRepository + Send + Sync>> = Arc::new(Box::new(
            PostgresCommandRepository::new(Arc::clone(&postgres)),
        ));

        let command_service: Arc<Box<dyn CommandService + Send + Sync>> = Arc::new(Box::new(
          CommandServiceImpl::new(Arc::clone(&command_repository)),
      ));
      let action_service: Arc<Box<dyn ActionService + Send + Sync>> = Arc::new(Box::new(
          ActionServiceImpl::new(Arc::clone(&action_repository), Arc::clone(&command_service)),
      ));

      let grpc_scheduler_client: Box<dyn SchedulerClient + Send + Sync> = Box::new(GrpcSchedulerClient::new(grpc_url).await.unwrap());
      let scheduler_client: Arc<Mutex<Box<dyn SchedulerClient + Send + Sync>>> = Arc::new(Mutex::new(grpc_scheduler_client));

      let scheduler_service: Arc<Box<dyn SchedulerService + Send + Sync>> = Arc::new(Box::new(SchedulerServiceImpl::new(
          Arc::clone(&action_service),
          Arc::clone(&scheduler_client),
      )));
      let pipeline_service: Arc<Box<dyn PipelineService + Send + Sync>> =
          Arc::new(Box::new(PipelineServiceImpl::new(
              Arc::clone(&pipeline_repository),
              Arc::clone(&action_service),
          )));

      Self {
          pipeline_service,
          action_service,
          command_service,
          scheduler_service,
      }
    }
}
