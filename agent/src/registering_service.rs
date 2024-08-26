use proto::RegisterAgentResponse;

use crate::AGENT_ID;

mod proto {
    tonic::include_proto!("scheduler");
}

pub async fn register_agent(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    match proto::agent_client::AgentClient::connect(url.to_string()).await {
        Ok(mut cli) => {
            //TODO: Use real health data
            let req = proto::Health {
                cpu_avail: 1,
                memory_avail: 1,
            };
            let request = tonic::Request::new(req);
            let response: RegisterAgentResponse = cli.register_agent(request).await?.into_inner();
            let mut agent_id = AGENT_ID.lock()?;
            *agent_id = response.id as i32;
            println!("This agent will get the id : {:?}", response.id);
            return Ok(());
        }
        Err(err) => {
            return Err(Box::new(err));
        }
    };

}
