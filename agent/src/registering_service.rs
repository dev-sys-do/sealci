mod proto {
    tonic::include_proto!("scheduler");
}

pub async fn register_agent(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    match proto::agent_client::AgentClient::connect(url.to_string()).await {
        Ok(mut cli) => {
            //TODO: Use real health data
            let req = proto::Health {
                cpu_usage: 1,
                memory_usage: 1,
            };
            let request = tonic::Request::new(req);
            cli.register_agent(request).await.unwrap();
            return Ok(());
        }
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    // let mut tries = 0;
    // while tries != 5 {
    //     println!("Try number: {}", tries);
    //     client = match proto::agent_client::AgentClient::connect(url).await {
    //         Ok(mut cli) => {
    //             println!("Connection succeeded");
    //             //TODO: Use real health data
    //             let req = proto::Health {
    //                 cpu_usage: 1,
    //                 memory_usage: 1,
    //             };
    //             let request = tonic::Request::new(req);
    //             cli.register_agent(request).await.unwrap();
    //             cli
    //         }
    //         Err(err) => {
    //             if tries == 4 {
    //                 println!("Connection failed: {:?}", err);
    //                 return Err(Box::new(err));
    //             } else {
    //                 println!("Connection failed: {:?}", err);
    //                 tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    //                 tries += 1;
    //                 continue;
    //             }
    //         }
    //     };
    // }
}
