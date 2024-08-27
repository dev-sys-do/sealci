use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

use actix_web::{web::Data, App, HttpServer};
use parser::pipe_parser::MockManifestParser;
use pipeline::pipeline_controller;
use tracing::info;
use dotenv::dotenv;
use crate::database::database::Database;
use crate::env::Env;

pub mod grpc_scheduler {
    tonic::include_proto!("scheduler");
}

pub mod parser;
mod pipeline;
pub mod scheduler;
mod storage;
mod tests;
mod env;
mod database;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = ("0.0.0.0:4000".to_string()))]
    http: String,

    #[arg(long, default_value_t = ("http://0.0.0.0:55001".to_string()))]
    grpc: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = Args::parse();
    let env = Env::parse();

    let database = Database::new(&env.database_url).await;

    sqlx::migrate!("").run(&database.pool).await?;

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
