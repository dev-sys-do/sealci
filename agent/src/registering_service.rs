use tonic::transport::Channel;

use crate::proto::{agent_client, Health, RegisterAgentResponse};

pub async fn register_agent(
    url: &String,
) -> Result<(agent_client::AgentClient<Channel>, u32), Box<dyn std::error::Error>> {
    let mut cli: agent_client::AgentClient<tonic::transport::Channel> =
        match agent_client::AgentClient::connect(url.to_string()).await {
            Ok(client) => client,

            Err(err) => {
                return Err(Box::new(err));
            }
        };
    let req = Health {
        cpu_avail: 1,
        memory_avail: 1,
    };
    let request = tonic::Request::new(req);
    let response: RegisterAgentResponse = cli.register_agent(request).await?.into_inner();

    println!("This agent will get the id : {:?}", response.id);

    Ok((cli, response.id))
}
