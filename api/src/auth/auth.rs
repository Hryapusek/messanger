include!("../protobufs/auth.rs");

#[derive(Clone, Debug)]
pub struct AuthServiceService {}

use crate::auth::service::*;
use database_api::api::users::*;
use log::warn;
use tonic::*;
use crate::auth::utils::hash_password;
use super::utils::create_token;

#[tonic::async_trait]
impl auth_service_server::AuthService for AuthServiceService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<()>, Status> {
        if request.get_ref().user.is_none() {
            return Err(Status::invalid_argument("None of email or phone"));
        }

        let user;

        match request.get_ref().user.as_ref().unwrap() {
            register_user_request::User::Email(email) => {
                user = register_by_email(email, request.get_ref())?
            }
            register_user_request::User::Phone(phone) => {
                user = register_by_phone(phone, request.get_ref())?
            }
        };

        println!("{:?}", request.metadata());
        let mut response = Response::new(());
        Ok(response)
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        if request.get_ref().user.is_none() {
            return Err(Status::invalid_argument("None of email or phone"));
        }

        let user;

        match request.get_ref().user.as_ref().unwrap() {
            login_request::User::Email(email) => {
                user = get_user_by_email(email).map_err(|_| Status::internal("Database error"))?
            }
            login_request::User::Phone(phone) => {
                user = get_user_by_phone(phone).map_err(|_| Status::internal("Database error"))?
            }
        };

        if user.is_none() {
            return Err(Status::not_found("User not found"));
        }

        if user.as_ref().unwrap().password != hash_password(&request.get_ref().password) {
            return Err(Status::unauthenticated("Invalid password"));
        }

        let token = create_token(&user.unwrap());
        if let Err(e) = token {
            warn!("Failed to create token: {}", e);
            return Err(Status::internal(e));
        }

        Ok(Response::new(LoginResponse {
            token: token.unwrap(),
        }))
    }
}
