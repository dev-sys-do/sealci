use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use tonic::transport::Server;
use tracing::info;
use sealcid_traits::status::Status;
use crate::{
    errors::Error,
    interfaces::server::{agent_interface::AgentService, controller_interface::ControllerService},
    logic::agent_pool_logic::AgentPool,
    proto::{
        self,
        scheduler::{agent_server::AgentServer, controller_server::ControllerServer},
    },
};

use crate::config::Config;

#[derive(Clone)]
pub struct App {
    agent: AgentService,
    controller: ControllerService,
    config: Config,
    app_process: Arc<RwLock<tokio::task::JoinHandle<Result<(), Error>>>>,
}

impl sealcid_traits::App<Config> for App {
    type Error = Error;

    async fn run(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        let app_clone = self.clone();
        let mut process = app_process.write().await;
        *process = tokio::spawn(async move {
            let _ = app_clone.start();
            Ok(())
        });
        Ok(())
    }

    async fn configure(config: Config) -> Result<Self, Error> {
        Self::init(config)
    }

    async fn stop(&self) -> Result<(), Error> {
        let app_process = self.app_process.clone();
        let process = app_process.read().await;
        process.abort();
        Ok(())
    }

    async fn configuration(&self) -> Result<impl std::fmt::Display, Error> {
        Ok(self.config.clone())
    }

    async fn status(&self) -> Status {
        let app_process = self.app_process.read().await;
        if app_process.is_finished() {
            // Try to get the result without blocking
            Status::Stopped
        } else {
            Status::Running
        }
    }

    fn name(&self) -> String {
        "Scheduler".to_string()
    }
}


impl App {
    pub fn init(config: Config) -> Result<Self, Error> {
        // Initialize logger.
        tracing_subscriber::fmt::init();

        // Initializes the Agent Pool. They are lost when the Scheduler dies.
        let agent_pool = Arc::new(Mutex::new(AgentPool::new()));

        // Pass the shared Agent Pool to Agent and Controller services.
        let agent = AgentService::new(agent_pool.clone());
        let controller = ControllerService::new(agent_pool.clone());

        Ok(App {
            agent,
            controller,
            config,
            app_process: Arc::new(RwLock::new(tokio::spawn(async { Ok(()) }))),
        })
    }

    pub async fn start(&self) -> Result<(), Error> {
        let service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
            .build_v1()
            .map_err(|e| Error::GrpcSetupError(tonic::Status::internal(e.to_string())))?;

        info!("Starting gRPC server at {}", self.config.addr);
        Server::builder()
            .add_service(service)
            .add_service(AgentServer::new(self.agent.clone()))
            .add_service(ControllerServer::new(self.controller.clone()))
            .serve(self.config.addr.parse().map_err(|e| Error::AddrParseError(e))?)
            .await
            .map_err(|e| Error::GrpcServerError(tonic::Status::internal(e.to_string())))?;
        Ok(())
    }
}
