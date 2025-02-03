include!("../protobufs/auth.rs");

#[derive(Clone, Debug)]
pub struct AuthServiceService {}

use super::utils::{create_access_token, create_refresh_token};
use crate::auth::service::*;
use crate::auth::utils::hash_password;
use database_api::api::refresh_token::{delete_refresh_token, get_refresh_token_by_value};
use database_api::api::users::*;
use log::{error, info, trace, warn};
use tonic::*;

#[tonic::async_trait]
impl auth_service_server::AuthService for AuthServiceService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<()>, Status> {
        if request.get_ref().user.is_none() {
            trace!("None of email or phone were specified");
            return Err(Status::invalid_argument("None of email or phone"));
        }

        let user;

        match request.get_ref().user.as_ref().unwrap() {
            register_user_request::User::Email(email) => {
                user = register_by_email(email, request.get_ref()).map_err(|e| {
                    trace!("{}", e);
                    e
                })?;
            }
            register_user_request::User::Phone(phone) => {
                user = register_by_phone(phone, request.get_ref()).map_err(|e| {
                    trace!("{}", e);
                    e
                })?;
            }
        };

        log::trace!("User successfully registered: {:#?}", user);

        Ok(Response::new(()))
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
                user = get_user_by_email(email).map_err(|e| {
                    warn!("Error while retrieving email from database: {}", e);
                    Status::internal("Database error")
                })?;
            }
            login_request::User::Phone(phone) => {
                user = get_user_by_phone(phone).map_err(|e| {
                    warn!("Error while retrieving email from database: {}", e);
                    Status::internal("Database error")
                })?;
            }
        };

        if user.is_none() {
            trace!("User with data {:#?} was not found in database", request.get_ref().user.as_ref());
            return Err(Status::not_found("User not found"));
        }

        if user.as_ref().unwrap().password != hash_password(&request.get_ref().password) {
            return Err(Status::unauthenticated("Invalid password"));
        }

        let access_token = create_access_token(&user.as_ref().unwrap());
        if let Err(e) = access_token {
            log::error!("Failed to create token: {}", e);
            return Err(Status::internal(""));
        }

        let refresh_token = create_refresh_token(&user.as_ref().unwrap());
        if let Err(e) = refresh_token {
            log::error!("Failed to create refresh token: {}", e);
            return Err(Status::internal(""));
        }

        log::trace!("User logged in successfully: {:#?}", user.as_ref().unwrap());

        Ok(Response::new(LoginResponse {
            access_token: access_token.unwrap(),
            refresh_token: refresh_token.unwrap().token,
        }))
    }

    async fn login_refresh_token(
        &self,
        request: Request<LoginRefreshTokenRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let found_token = get_refresh_token_by_value(&request.get_ref().refresh_token);
        if let Err(e) = found_token {
            log::error!("Failed to get refresh token: {}", e);
            return Err(Status::internal(""));
        }

        let found_token = found_token.unwrap();
        if let None = found_token {
            trace!("Refresh token was not found by value {:#?}", request.get_ref().refresh_token);
            return Err(Status::not_found("Refresh token not found"));
        }
        let found_token = found_token.unwrap();
        let user = get_user(found_token.user_id);
        if let Err(e) = user {
            log::error!("Failed to get user: {}", e);
            return Err(Status::internal(""));
        }
        let user = user.unwrap();
        if let None = user {
            log::error!(
                "User was not found by refresh token user_id field. Refresh token id {}",
                found_token.id
            );
            return Err(Status::internal("User not found"));
        }
        let user = user.unwrap();

        let access_token = create_access_token(&user);
        if let Err(e) = access_token {
            log::error!("Failed to create token: {}", e);
            return Err(Status::internal(""));
        }

        let refresh_token = create_refresh_token(&user);
        if let Err(e) = refresh_token {
            log::error!("Failed to create refresh token: {}", e);
            return Err(Status::internal(""));
        }

        if let Err(e) = delete_refresh_token(found_token.id) {
            log::error!("Failed to delete refresh token: {}", e);
        }

        log::trace!("User logged in successfully by refresh token: {:#?}", user);

        Ok(Response::new(LoginResponse {
            access_token: access_token.unwrap(),
            refresh_token: refresh_token.unwrap().token,
        }))
    }
}
