use action::action_service::ActionService;
use clap::Parser;
use command::command_service::CommandService;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::database::database::Database;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use parser::pipe_parser::PipeParser;
use pipeline::pipeline_controller;
use tracing::info;

pub mod grpc_scheduler {
    tonic::include_proto!("scheduler");
}

mod action;
mod command;
mod database;
mod docs;
mod logs;
pub mod parser;
mod pipeline;
pub mod scheduler;
mod domain;
mod infrastructure;
mod tests;
mod health;

#[derive(Parser, Debug)]
struct Args {
    #[clap(env, long)]
    pub http: String,

    #[clap(env, long)]
    pub database_url: String,

    #[clap(env, long)]
    pub grpc: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = Args::parse();

    println!("${:?}", args);
    let database = Database::new(&args.database_url).await;

    let pool = Arc::new(database.pool);

    let addr_in = args.http;
    let grpc_scheduler = args.grpc;

    tracing_subscriber::fmt::init();

    let client = Arc::new(Mutex::new(
        grpc_scheduler::controller_client::ControllerClient::connect(grpc_scheduler)
            .await
            .expect("Failed to connect to controller"),
    ));

    let command_service = Arc::new(CommandService::new(Arc::clone(&pool)));

    let action_service = Arc::new(ActionService::new(
        Arc::clone(&pool),
        Arc::clone(&command_service),
    ));

    let scheduler_service = Arc::new(scheduler::SchedulerService::new(
        client.clone(),
        Arc::new(logs::log_repository::LogRepository::new(Arc::clone(&pool))),
        Arc::clone(&action_service),
    ));

    let parser_service = Arc::new(PipeParser {});

    let pipeline_service = Arc::new(pipeline::pipeline_service::PipelineService::new(
        scheduler_service.clone(),
        parser_service.clone(),
        Arc::clone(&pool),
        Arc::clone(&action_service),
    ));

    info!("Listenning on {}", addr_in);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(pipeline_service.clone())) // TODO: replace this implementation by the real parser
            .app_data(Data::new(Arc::clone(&action_service)))
            .service(pipeline_controller::create_pipeline)
            .service(pipeline_controller::get_pipelines)
            .service(pipeline_controller::get_pipeline)
            .service(docs::doc)
            .service(docs::openapi)
            .route("/health", actix_web::web::get().to(health::handlers::health_check))
    })
    .bind(addr_in)?
    .workers(1)
    .run()
    .await
}
