use std::sync::Arc;

use actix_web::{web::Data, App, HttpServer};
use parser::pipe_parser::MockManifestParser;
use pipeline::pipeline_controller;
use tracing::info;

pub mod scheduler {
    tonic::include_proto!("scheduler");
}

pub mod parser;
mod pipeline;
mod storage;

mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    let addr_in = "0.0.0.0:4000";

    tracing_subscriber::fmt::init();

    let client = Arc::new(
        scheduler::controller_client::ControllerClient::connect("http://0.0.0.0:50051")
            .await
            .expect("Failed to connect to controller"),
    );

    let parser_service = Arc::new(MockManifestParser {});

    let pipeline_service = Arc::new(pipeline::pipeline_service::PipelineService::new(
        client.clone(),
        parser_service.clone(),
    ));

    info!("Listenning on {}", addr_in);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pipeline_service.clone())) // TODO: replace this implementation by the real parser
            .service(pipeline_controller::create_pipeline)
    })
    .bind(addr_in)?
    .run()
    .await
}
