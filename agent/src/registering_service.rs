use proto::RegisterAgentResponse;

mod proto {
    tonic::include_proto!("scheduler");
}

pub async fn register_agent(url: &String) -> Result<u32, Box<dyn std::error::Error>> {
    match proto::agent_client::AgentClient::connect(url.to_string()).await {
        Ok(mut cli) => {
            //TODO: Use real health data
            let req = proto::Health {
                cpu_avail: 1,
                memory_avail: 1,
            };
            let request = tonic::Request::new(req);
            let response: RegisterAgentResponse = cli.register_agent(request).await?.into_inner();
            let agent_id = response.id;
            println!("This agent will get the id : {:?}", response.id);
            return Ok(agent_id);
        }
        Err(err) => {
            return Err(Box::new(err));
        }
    };
}
