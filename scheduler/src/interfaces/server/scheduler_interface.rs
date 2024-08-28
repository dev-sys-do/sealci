use super::{agent_interface::AgentService, controller_interface::ControllerService};

pub struct SchedulerService {
    agent_service: AgentService,
    controller_service: ControllerService,
}

impl SchedulerService {
    pub fn new(agent_service: AgentService, controller_service: ControllerService) -> Self {
        Self {
            agent_service: agent_service,
            controller_service: controller_service,
        }
    }
}

#[tonic::async_trait]
impl Scheduler for SchedulerService {
    
}