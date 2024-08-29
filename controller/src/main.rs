use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::database::database::Database;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use parser::pipe_parser::MockManifestParser;
use pipeline::pipeline_controller;
use tracing::info;

pub mod grpc_scheduler {
    tonic::include_proto!("scheduler");
}

mod database;
mod docs;
pub mod parser;
mod pipeline;
pub mod scheduler;
mod tests;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = ("0.0.0.0:4000".to_string()))]
    http: String,

    #[clap(env, long)]
    pub database_url: String,

    #[arg(long, default_value_t = ("http://0.0.0.0:55001".to_string()))]
    grpc: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let database = Database::new(&args.database_url).await;

    sqlx::migrate!("./migrations")
        .run(&database.pool)
        .await
        .unwrap();

    let pool = Arc::new(database.pool);

    let addr_in = args.http;
    let grpc_scheduler = args.grpc;

    tracing_subscriber::fmt::init();

    let client = Arc::new(Mutex::new(
        grpc_scheduler::controller_client::ControllerClient::connect(grpc_scheduler)
            .await
            .expect("Failed to connect to controller"),
    ));

    let scheduler_service = Arc::new(scheduler::SchedulerService::new(client.clone()));

    let parser_service = Arc::new(MockManifestParser {});

    let pipeline_service = Arc::new(pipeline::pipeline_service::PipelineService::new(
        scheduler_service.clone(),
        parser_service.clone(),
        Arc::clone(&pool)
    ));

    info!("Listenning on {}", addr_in);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pipeline_service.clone())) // TODO: replace this implementation by the real parser
            .service(pipeline_controller::create_pipeline)
            .service(docs::doc)
            .service(docs::openapi)
    })
    .bind(addr_in)?
    .run()
    .await
}
