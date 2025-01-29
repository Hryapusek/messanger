include!("protobufs/auth.rs");

#[derive(Clone, Debug)]
pub struct AuthServiceService {}

use tonic::*;

#[tonic::async_trait]
impl auth_service_server::AuthService for AuthServiceService {
  async fn hello_world(&self, request: Request<()>) -> Result<Response<HelloWorldResponse>, Status> {
    println!("{:?}", request.metadata());
    let mut response = Response::new(HelloWorldResponse { hello: "Hello, World!".to_string() });
    response.metadata_mut().append("access_token", "1234".parse().unwrap());
    Ok(response)
  }
}