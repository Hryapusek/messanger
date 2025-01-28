include!("protobufs/auth.rs");

#[derive(Clone, Debug)]
pub struct AuthServiceService {}

use tonic::*;

#[tonic::async_trait]
impl auth_service_server::AuthService for AuthServiceService {
  async fn hello_world(&self, _request: Request<()>) -> Result<Response<HelloWorldResponse>, Status> {
    Ok(Response::new(HelloWorldResponse { hello: "Hello, World!".to_string() }))
  }
}