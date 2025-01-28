mod auth;
use std::str::FromStr;

use auth::AuthServiceService;
use auth::auth_service_server::AuthServiceServer;
use tonic::transport::Server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = AuthServiceServer::new(AuthServiceService{});
    Server::builder().add_service(server).serve(std::net::SocketAddr::from_str("0.0.0.0:50051").unwrap()).await?;
    Ok(())
}
