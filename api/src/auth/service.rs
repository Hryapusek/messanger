use crate::auth::auth::RegisterUserRequest;
use database_api::{
    api::users::{create_user, get_user_by_email, get_user_by_phone}, models::user::{NewUser, User},
};
use tonic::*;
use log::{warn, error, info};

use super::utils::{check_password, hash_password};

pub fn register_by_email(email: &str, request: &RegisterUserRequest) -> Result<User, Status> {
    if let Some(_) = get_user_by_email(email).map_err(|_| Status::internal("Database error"))? {
        return Err(Status::already_exists(
            "User with this email already exists",
        ));
    }

    check_password(&request.password)?;

    let result = create_user(NewUser {
        email: email.to_string(),
        password: hash_password(&request.password),
        ..Default::default()
    });

    if result.is_err() {
        error!("Error creating user: {}", result.err().unwrap());
        return Err(Status::internal("Database error. "));
    }

    Ok(result.unwrap())
}

pub fn register_by_phone(phone: &str, request: &RegisterUserRequest) -> Result<User, Status> {
  if let Some(_) = get_user_by_phone(&phone).map_err(|_| Status::internal("Database error"))? {
      return Err(Status::already_exists(
          "User with this email already exists",
      ));
  }

  check_password(&request.password)?;

  let result = create_user(NewUser {
      phone: Some(phone.to_owned()),
      password: hash_password(&request.password),
      ..Default::default()
  });

  if result.is_err() {
      error!("Error creating user: {}", result.err().unwrap());
      return Err(Status::internal("Database error. "));
  }

  Ok(result.unwrap())
}
