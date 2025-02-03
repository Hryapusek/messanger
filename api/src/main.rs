mod auth;
use std::str::FromStr;

use auth::auth::AuthServiceService;
use auth::auth::auth_service_server::AuthServiceServer;
use tonic::transport::Server;
use database_api::api::refresh_token::*;
use dotenv::dotenv;
use env_logger::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv();
    init();
    let server = AuthServiceServer::new(AuthServiceService{});
    Server::builder().add_service(server).serve(std::net::SocketAddr::from_str("0.0.0.0:50051").unwrap()).await?;
    Ok(())
}
