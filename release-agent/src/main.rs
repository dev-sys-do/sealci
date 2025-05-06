use tonic::transport::Server;

mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = config::App::parse();

    tracing_subscriber::fmt::init();

    // need to add services
    Server::builder().serve(args.grpc).await?;
}
