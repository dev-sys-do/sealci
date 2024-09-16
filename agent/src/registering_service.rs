use crate::proto::{agent_client, Health, Hostname, RegisterAgentRequest, RegisterAgentResponse};
use sysinfo::System;
use tonic::transport::Channel;

pub async fn register_agent(
    scheduler_url: &String,
    agent_host: &String,
    port: u32,
) -> Result<(agent_client::AgentClient<Channel>, u32), Box<dyn std::error::Error>> {
    let mut cli: agent_client::AgentClient<tonic::transport::Channel> =
        match agent_client::AgentClient::connect(scheduler_url.to_string()).await {
            Ok(client) => client,

            Err(err) => {
                return Err(Box::new(err));
            }
        };
    let sys = System::new_all();
    let health = Health {
        cpu_avail: 100 - sys.global_cpu_info().cpu_usage() as u32,
        memory_avail: (sys.total_memory() - sys.used_memory()) as u64,
    };

    let host = Hostname {
        host: agent_host.to_string(),
        port,
    };
    let req = RegisterAgentRequest {
        health: Some(health),
        hostname: Some(host),
    };
    let request = tonic::Request::new(req);
    let response: RegisterAgentResponse = cli.register_agent(request).await?.into_inner();
    Ok((cli, response.id))
}
