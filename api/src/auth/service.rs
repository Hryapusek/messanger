use crate::auth::auth::RegisterUserRequest;
use database_api::{
    api::users::get_user_by_email, api::users::get_user_by_phone, models::user::User,
};
use tonic::*;

pub fn register_by_email(email: &str, request: &RegisterUserRequest) -> Result<User, Status> {
    if let Some(user) = get_user_by_email(email).map_err(|_| Status::internal("Database error"))? {
        return Err(Status::already_exists(
            "User with this email already exists",
        ));
    }

    todo!("")
}

pub fn register_by_phone(phone: &str, request: &RegisterUserRequest) -> Result<User, Status> {
    todo!("")
}
