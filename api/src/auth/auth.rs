include!("../protobufs/auth.rs");

#[derive(Clone, Debug)]
pub struct AuthServiceService {}

use tonic::*;
use crate::auth::service::*;

#[tonic::async_trait]
impl auth_service_server::AuthService for AuthServiceService {
  async fn register_user(&self, request: Request<RegisterUserRequest>) -> Result<Response<RegisterResponse>, Status> {
    if request.get_ref().user.is_none() {
      return Err(Status::invalid_argument("None of email or phone"));
    }

    match request.get_ref().user.as_ref().unwrap() {
      register_user_request::User::Email(email) => {
        register_by_email(email, request.get_ref())
      }
      register_user_request::User::Phone(phone) => {
        register_by_phone(phone, request.get_ref())
      }
    };
    
    println!("{:?}", request.metadata());
    let mut response = Response::new(RegisterResponse { token: "1234".to_string() });
    response.metadata_mut().append("access_token", "1234".parse().unwrap());
    Ok(response)
  }
}