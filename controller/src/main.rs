use action::action_service::ActionService;
use application::app_context::AppContext;
use clap::Parser;
use command::command_service::CommandService;
use infrastructure::{config::Env, db::postgres::Postgres};
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
mod application;
mod command;
mod database;
mod docs;
mod domain;
mod infrastructure;
mod logs;
pub mod parser;
mod pipeline;
pub mod scheduler;
mod tests;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Env::parse();
    println!("env: {:?}", env);
    // let args = Args::parse();

    // let database = Database::new(&args.database_url).await;

    // sqlx::migrate!("./migrations")
    //     .run(&database.pool)
    //     .await
    //     .unwrap();

    // let pool = Arc::new(database.pool);

    // let addr_in = args.http;
    // let grpc_scheduler = args.grpc;

    let postgres = Arc::new(
        Postgres::new(&env.database_url)
        .await
    );

    let app_context = AppContext::initialize(Arc::clone(&postgres), &env.grpc).await;


    // let client = Arc::new(Mutex::new(
    //     grpc_scheduler::controller_client::ControllerClient::connect(grpc_scheduler)
    //         .await
    //         .expect("Failed to connect to controller"),
    // ));

    // let command_service = Arc::new(CommandService::new(Arc::clone(&pool)));

    // let action_service = Arc::new(ActionService::new(
    //     Arc::clone(&pool),
    //     Arc::clone(&command_service),
    // ));

    // let scheduler_service = Arc::new(scheduler::SchedulerService::new(
    //     client.clone(),
    //     Arc::new(logs::log_repository::LogRepository::new(Arc::clone(&pool))),
    //     Arc::clone(&action_service),
    // ));

    // let parser_service = Arc::new(PipeParser {});

    // let pipeline_service = Arc::new(pipeline::pipeline_service::PipelineService::new(
    //     scheduler_service.clone(),
    //     parser_service.clone(),
    //     Arc::clone(&pool),
    //     Arc::clone(&action_service),
    // ));

    info!("Listenning on {}", "0.0.0.0:3333");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // .app_data(Data::new(pipeline_service.clone())) // TODO: replace this implementation by the real parser
            // .app_data(Data::new(Arc::clone(&action_service)))
            // .service(pipeline_controller::create_pipeline)
            // .service(pipeline_controller::get_pipelines)
            // .service(pipeline_controller::get_pipeline)
            .service(docs::doc)
            .service(docs::openapi)
    })
    .bind("0.0.0.0:3333")?
    .workers(1)
    .run()
    .await
}
