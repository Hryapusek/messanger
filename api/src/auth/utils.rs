use std::collections::BTreeMap;

use crate::auth::constants::{REFRESH_TOKEN_DAYS_TO_EXPIRE, REFRESH_TOKEN_LENGTH, USER_ID};
use chrono::{self, Duration};
use database_api::models::{refresh_token::{NewRefreshToken, RefreshToken}, user::User};
use hmac::{Hmac, Mac};
use jwt::*;
use rand::distr::SampleString;
use rand_distr::Alphanumeric;
use sha2::{Digest, Sha256};
use tonic::Status;

use super::constants::{EXPIRE_AT, JWT_DAYS_TO_EXPIRE};

pub fn check_password(password: &str) -> Result<(), Status> {
    const MIN_PASSWORD_LENGTH: usize = 8;

    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(Status::invalid_argument("Password is too short"));
    }

    Ok(())
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = sha2::Sha256::default();
    hasher.update(&password.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn create_access_token(user: &User) -> Result<String, String> {
    let mut claims = BTreeMap::new();
    claims.insert(USER_ID, user.id.to_string());
    let today = chrono::Utc::now().naive_utc();
    let expire_at = today + chrono::Duration::days(JWT_DAYS_TO_EXPIRE);
    claims.insert(EXPIRE_AT, expire_at.and_utc().timestamp().to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("SECRET_JWT_KEY")
            .expect("Secret key is not set")
            .as_bytes(),
    )
    .map_err(|e| e.to_string())?;
    claims.sign_with_key(&key).map_err(|e| e.to_string())
}

pub fn create_refresh_token(user: &User) -> Result<RefreshToken, String> {
    let refresh_token_string = Alphanumeric.sample_string(&mut rand::rng(), REFRESH_TOKEN_LENGTH);

    let result = NewRefreshToken {
        user_id: user.id,
        token: refresh_token_string,
        created_at: None,
        expires_at: chrono::Utc::now().naive_utc() + Duration::days(REFRESH_TOKEN_DAYS_TO_EXPIRE)
    };

    database_api::api::refresh_token::create_refresh_token(&result)
}
