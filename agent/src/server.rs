use futures::Stream;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

use crate::health::health_server::Health;
use crate::health::{
    MetricRequest, MetricReply, Metric,
};

#[derive(Debug)]
pub struct HealthCheck {
    health: Arc<Mutex<HashMap<String, Metric>>>,
}

impl Default for HealthCheck {
    fn default() -> Self {
        HealthCheck {
            health: Arc::new(Mutex::new(HashMap::<String, Metric>::new())),
        }
    }
}

#[tonic::async_trait]
impl Health for HealthCheck {

    async fn get(&self, request: Request<MetricRequest>) -> Result<Response<MetricReply>, Status> {
        let request = request.into_inner();

        print!("Request: {:?}", request);

        let mut system = System::new();
        system.refresh_all();

        let metric = MetricReply {
            name: format!("{}", System::name().unwrap_or_default()),
            kernel_version: format!("{}", System::kernel_version().unwrap_or_default()),
            os_version: format!("{}", System::os_version().unwrap_or_default()),
            host_name: format!("{}", System::host_name().unwrap_or_default()),
            memory: format!("{:.2} GB", system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
            used_memory: format!("{:.2} GB", system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
            percent_memory: format!("{:.2}%", system.used_memory() as f32 / system.total_memory() as f32 * 100.0),
            cpu: format!("{}", system.cpus().len()),
        };

        Ok(Response::new(metric.clone()))
    }

    type WatchStream = Pin<Box<dyn Stream<Item = Result<MetricReply, Status>> + Send>>;

    async fn watch(
        &self,
        request: Request<MetricRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        // Récupérer les métriques initiales
        let mut metric = self.get(request).await?.into_inner();

        // Le canal pour envoyer les mises à jour des métriques au client
        let (tx, rx) = mpsc::unbounded_channel();

        // Cloner la référence pour l'utiliser dans la tâche asynchrone
        let health = self.health.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;

                let mut system = System::new();
                system.refresh_all();

                let metric_refresh = MetricReply {
                    name: System::name().unwrap_or_default(),
                    kernel_version: System::kernel_version().unwrap_or_default(),
                    os_version: System::os_version().unwrap_or_default(),
                    host_name: System::host_name().unwrap_or_default(),
                    memory: format!("{:.2} GB", system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                    used_memory: format!("{:.2} GB", system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0),
                    percent_memory: format!("{:.2}%", system.used_memory() as f32 / system.total_memory() as f32 * 100.0),
                    cpu: format!("{}", system.cpus().len()),
                };

                if metric_refresh != metric {
                    if let Err(err) = tx.send(Ok(metric_refresh.clone())) {
                        println!("ERROR: failed to update stream client: {:?}", err);
                        return;
                    }
                    metric = metric_refresh;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::WatchStream))
    }
}