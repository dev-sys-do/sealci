use std::error::Error;
use sysinfo::System;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::Request;
use tracing::{error, info};

use crate::proto::agent_client::AgentClient;
use crate::proto::{Health, HealthStatus};

pub(crate) async fn report_health(
    client: &mut AgentClient<tonic::transport::Channel>,
    agent_id: u32,
) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::unbounded_channel();
    let stream = UnboundedReceiverStream::new(rx);

    let mut previous_usage = HealthStatus {
        agent_id,
        health: Some(Health {
            cpu_avail: 0,
            memory_avail: 0,
        }),
    };

    let mut system = System::new_all();
    tokio::spawn(async move {
        loop {
            // Fetch current usage
            let current_health = get_current_health_status(&mut system, agent_id);

            // Check if the change is significant
            if has_significant_change(&previous_usage.health, &current_health.health, 5.0) {
                previous_usage = current_health;
                let _ = tx.send(current_health);
                info!("Health status sent: {:?}", current_health);
                info!("Health status sent: {:?}", current_health);
            }

            // Delay before next check
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    match client.report_health_status(Request::new(stream)).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error: {:?}", err);
            return Err(Box::new(err));
        }
    };
    Ok(())
}

fn get_current_health_status(sys: &mut System, agent_id: u32) -> HealthStatus {
    sys.refresh_all();
    let cpu_avail = 100 - sys.global_cpu_info().cpu_usage() as u32;
    let memory_avail = (sys.total_memory() - sys.used_memory()) as u64;

    HealthStatus {
        agent_id: agent_id,
        health: Some(Health {
            cpu_avail,
            memory_avail,
        }),
    }
}

fn has_significant_change(prev: &Option<Health>, current: &Option<Health>, threshold: f32) -> bool {
    if let (Some(prev), Some(current)) = (prev, current) {
        let cpu_change = (current.cpu_avail as f32 - prev.cpu_avail as f32).abs();
        let memory_change = ((current.memory_avail as f32 - prev.memory_avail as f32)
            / prev.memory_avail as f32
            * 100.0)
            .abs();
        return cpu_change >= threshold || memory_change >= threshold;
    }
    false
}
