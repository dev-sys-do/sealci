use crate::common::proto::StatusResponse;
use crate::server::config::Update;
use crate::{
    common::{
        error::Error,
        proto::{
            AgentMutation, ControllerMutation, MonitorMutation, ReleaseAgentMutation,
            SchedulerMutation, ServiceStatusMessage, Services, StatusRequest,
            daemon_server::Daemon as DaemonGrpc,
        },
    },
    server::daemon::Daemon,
};
use agent::config::Config as AgentConfig;
use compactor::config::Config as ReleaseAgentConfig;
use controller::config::Config as ControllerConfig;
use monitor::config::Config as MonitorConfig;
use sealci_scheduler::config::Config as SchedulerConfig;
use sealcid_traits::App;
use tonic::{Request, Response, Status, async_trait};
use tracing::{debug, error};

#[async_trait]
impl DaemonGrpc for Daemon {
    async fn mutate_agent(&self, request: Request<AgentMutation>) -> Result<Response<()>, Status> {
        let new_config = request.into_inner();
        match new_config.toggle_agent {
            Some(true) => {
                self.agent
                    .enable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::ToggleAgentError(e)))?;
            }
            Some(false) => {
                // If toggle_agent is false, we disable the agent
                self.agent
                    .disable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::ToggleAgentError(e)))?;
            }
            None => {
                // If toggle_agent is None, we do nothing
            }
        }
        self.global_config.write().await.update(new_config);
        let global_config = self.global_config.read().await;

        let agent_config: AgentConfig = global_config.to_owned().into();
        self.agent
            .restart_with_config(agent_config.clone())
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartAgentError(e)))?;
        Ok(Response::new(()))
    }

    async fn mutate_release_agent(
        &self,
        request: Request<ReleaseAgentMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        let new_config = request.into_inner();
        if Some(false) == new_config.toggle_release_agent {
            self.release_agent
                .disable()
                .await
                .map_err(|e| Status::failed_precondition(Error::RestartReleaseAgentError(e)))?;
            return Ok(Response::new(()));
        }
        let mut global_config = self.global_config.write().await;
        global_config.update(new_config);
        let release_agent_config: ReleaseAgentConfig = global_config.to_owned().into();
        let me = self.clone();
        me.release_agent
            .restart_with_config(release_agent_config.clone())
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartReleaseAgentError(e)))?;
        Ok(Response::new(()))
    }

    async fn mutate_scheduler(
        &self,
        request: Request<SchedulerMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        let new_config = request.into_inner();
        match new_config.toggle_scheduler {
            Some(true) => {
                self.scheduler
                    .enable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::RestartSchedulerError(e)))?;
            }
            Some(false) => {
                // If toggle_scheduler is false, we disable the scheduler
                self.scheduler
                    .disable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::RestartSchedulerError(e)))?;
            }
            None => {
                // If toggle_scheduler is None, we do nothing
            }
        }
        self.global_config.write().await.update(new_config);
        let global_config = self.global_config.read().await;
        let scheduler_config: SchedulerConfig = global_config.to_owned().into();
        self.scheduler
            .restart_with_config(scheduler_config)
            .await
            .map_err(|e| {
                error!("Failed to restart scheduler: {:?}", e);
                Status::failed_precondition(Error::RestartSchedulerError(e))
            })?;
        let controller_config: ControllerConfig = global_config.to_owned().into();
        self.controller
            .restart_with_config(controller_config)
            .await
            .map_err(|e| {
                error!("Failed to restart controller: {:?}", e);
                Status::failed_precondition(Error::RestartControllerError(e))
            })?;
        Ok(Response::new(()))
    }

    async fn mutate_monitor(
        &self,
        request: Request<MonitorMutation>,
    ) -> std::result::Result<Response<()>, tonic::Status> {
        let new_config = request.into_inner();
        match new_config.toggle_monitor {
            Some(true) => {
                debug!("toggle_monitor: Some(true) - enabling monitor");
                self.monitor
                    .enable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::ToggleMonitorError(e)))?;
            }
            None => {
                debug!("toggle_monitor: None - doing nothing");
                // If toggle_monitor is None, we do nothing
            }
            Some(false) => {
                debug!("toggle_monitor: Some(false) - disabling monitor");
                // If toggle_monitor is false, we disable the monitor
                self.monitor
                    .disable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::ToggleMonitorError(e)))?;
            }
        }

        self.global_config.write().await.update(new_config);
        let monitor_config: MonitorConfig = self.global_config.read().await.to_owned().into();

        self.monitor
            .restart_with_config(monitor_config.clone())
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        Ok(Response::new(()))
    }

    async fn mutate_controller(
        &self,
        request: Request<ControllerMutation>,
    ) -> Result<Response<()>, tonic::Status> {
        let new_config = request.into_inner();
        match new_config.toggle_controller {
            Some(true) => {
                self.controller
                    .enable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
            }
            Some(false) => {
                // If toggle_controller is false, we disable the controller
                self.controller
                    .disable()
                    .await
                    .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
            }
            None => {}
        }

        let mut global_config = self.global_config.write().await;
        global_config.update(new_config);
        let controller_config: ControllerConfig = global_config.to_owned().into();
        self.controller
            .restart_with_config(controller_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
        let monitor_config: MonitorConfig = global_config.to_owned().into();
        self.monitor
            .restart_with_config(monitor_config)
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        Ok(Response::new(()))
    }

    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, tonic::Status> {
        let service = request.into_inner();
        let mut status: Vec<ServiceStatusMessage> = Vec::new();
        if None == service.status_type {
            let agent_status = self.agent.app.read().await.status().await;
            let controller_status = self.controller.app.read().await.status().await;
            let monitor_status = self.monitor.app.read().await.status().await;
            let scheduler_status = self.scheduler.app.read().await.status().await;
            status.push(ServiceStatusMessage {
                service: Services::Agent.into(),
                status: agent_status.into(),
            });
            status.push(ServiceStatusMessage {
                service: Services::Controller.into(),
                status: controller_status.into(),
            });
            status.push(ServiceStatusMessage {
                service: Services::Monitor.into(),
                status: monitor_status.into(),
            });
            status.push(ServiceStatusMessage {
                service: Services::Scheduler.into(),
                status: scheduler_status.into(),
            });

            return Ok(Response::new(StatusResponse { statuses: status }));
        }
        match Services::try_from(
            service
                .status_type
                .expect("Should not break since it is checked above"),
        ) {
            Ok(Services::Agent) => {
                let agent_status = self.agent.app.read().await.status().await;
                status.push(ServiceStatusMessage {
                    service: Services::Agent.into(),
                    status: agent_status.into(),
                });
            }
            Ok(Services::Controller) => {
                let controller_status = self.controller.app.read().await.status().await;
                status.push(ServiceStatusMessage {
                    service: Services::Controller.into(),
                    status: controller_status.into(),
                });
            }
            Ok(Services::Monitor) => {
                let monitor_status = self.monitor.app.read().await.status().await;
                status.push(ServiceStatusMessage {
                    service: Services::Monitor.into(),
                    status: monitor_status.into(),
                });
            }
            Ok(Services::Scheduler) => {
                let scheduler_status = self.scheduler.app.read().await.status().await;
                status.push(ServiceStatusMessage {
                    service: Services::Scheduler.into(),
                    status: scheduler_status.into(),
                });
            }
            _ => {
                return Err(Status::invalid_argument("Invalid service type specified"));
            }
        }
        Ok(Response::new(StatusResponse { statuses: status }))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        debug!("Starting Daemon gRPC service");
        let _ = request.into_inner(); // We don't use the request, but we need to consume it

        self.monitor
            .enable()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        self.monitor
            .restart()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        self.scheduler
            .enable()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartSchedulerError(e)))?;
        self.scheduler
            .restart()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartSchedulerError(e)))?;
        self.release_agent
            .enable()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartReleaseAgentError(e)))?;
        self.release_agent
            .restart()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartReleaseAgentError(e)))?;
        self.agent
            .enable()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartAgentError(e)))?;
        self.agent
            .restart()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartAgentError(e)))?;
        self.controller
            .enable()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
        self.controller
            .restart()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
        Ok(Response::new(()))
    }
    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        debug!("Stopping Daemon gRPC service");
        let _ = request.into_inner(); // We don't use the request, but we need to consume it
        self.controller
            .app
            .write()
            .await
            .stop()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartControllerError(e)))?;
        self.monitor
            .app
            .write()
            .await
            .stop()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartMonitorError(e)))?;
        self.scheduler
            .app
            .write()
            .await
            .stop()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartSchedulerError(e)))?;
        self.agent
            .app
            .write()
            .await
            .stop()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartAgentError(e)))?;
        self.release_agent
            .app
            .write()
            .await
            .stop()
            .await
            .map_err(|e| Status::failed_precondition(Error::RestartReleaseAgentError(e)))?;
        debug!("All services stopped successfully");
        Ok(Response::new(()))
    }
}
