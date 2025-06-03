use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use controller::application::app_context::AppContext;
use controller::application::http::pipeline::router::configure as configure_pipeline_routes;
use controller::application::services::action_service::ActionServiceImpl;
use controller::application::services::command_service::CommandServiceImpl;
use controller::application::services::pipeline_service::PipelineServiceImpl;
use controller::application::services::scheduler_service_impl::SchedulerServiceImpl;
use controller::infrastructure::db::postgres::Postgres;
use controller::infrastructure::grpc::grpc_scheduler_client::GrpcSchedulerClient;
use controller::infrastructure::repositories::action_repository::PostgresActionRepository;
use controller::infrastructure::repositories::command_repository::PostgresCommandRepository;
use controller::infrastructure::repositories::log_repository::PostgresLogRepository;
use controller::infrastructure::repositories::pipeline_repository::PostgresPipelineRepository;
use controller::{docs, health};
use dotenv::dotenv;
use futures::lock::Mutex;
use std::sync::Arc;
use tracing::info;

#[derive(Parser, Debug)]
struct Args {
    /// HTTP listen address (host:port).
    /// Precedence (see clap docs: defaults < env < CLI):
    /// 1. --http CLI flag
    /// 2. HTTP env var (from OS or .env)
    /// If unset, clap will report an error.
    /// Example values:
    ///   --http 127.0.0.1:8080
    ///   HTTP=0.0.0.0:8000
    #[clap(env, long)]
    pub http: String,

    /// Database connection string.
    /// Precedence:
    /// 1. --database-url CLI flag
    /// 2. DATABASE_URL env var
    /// If unset, clap will report an error.
    /// Example:
    ///   --database-url postgres://user:pass@localhost:5432/mydb
    #[clap(env, long)]
    pub database_url: String,

    /// gRPC scheduler service endpoint.
    /// Precedence:
    /// 1. --grpc CLI flag
    /// 2. GRPC env var
    /// If unset, clap will report an error.
    /// Example:
    ///   --grpc http://127.0.0.1:50051
    #[clap(env, long)]
    pub grpc: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Load .env file into OS environment variables before parsing clap args.
    // This ensures HTTP, DATABASE_URL, GRPC can be defined in .env and picked up here.
    dotenv().ok();

    // Parse command line arguments and environment variables into Args struct
    let args = Args::parse();

    // Debug print the provided arguments for verification
    println!("Parsed args: {:?}", args);

    // HTTP server binding address (host:port)
    let addr_in: String = args.http;

    // Initialize application context with database and gRPC service configurations
    let app_context: AppContext = AppContext::initialize(
        &args.database_url,
        &args.grpc,
    ).await.expect("REASON");

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();
    

    info!("Listening on {}", addr_in);
    
    // Start HTTP server with CORS, logging middleware, and configured routes
    HttpServer::new(move || {
        // Configure CORS to allow any origin/method/header, cache preflight for 1 hour
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            // Register application state data for pipeline, action, and scheduler services
            .app_data(Data::new(app_context.clone()))
            .configure(configure_pipeline_routes)
            // Add documentation and health check endpoints
            .service(docs::doc)
            .service(docs::openapi)
            .route(
                "/health",
                actix_web::web::get().to(health::handlers::health_check),
            )
    })
    .bind(addr_in)?
    .workers(1)
    .run()
    .await
}
