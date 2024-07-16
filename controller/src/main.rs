use actix_web::{web::Data, App, HttpServer};
use tracing::info;

pub mod scheduler {
    tonic::include_proto!("scheduler");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr_in = "0.0.0.0:4000";

    tracing_subscriber::fmt::init();

    let client = scheduler::controller_client::ControllerClient::connect("http:/[::1]:50051")
        .await
        .expect("Couldn't create grpc client");

    info!("Listenning on {}", addr_in);

    HttpServer::new(move || App::new().app_data(Data::new(client.clone())))
        .bind(addr_in)?
        .run()
        .await
}
