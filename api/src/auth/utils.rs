use std::collections::BTreeMap;

use database_api::models::user::User;
use hmac::{Hmac, Mac};
use jwt::*;
use sha2::{Sha256, Digest};
use tonic::Status;
use chrono;
use crate::auth::constants::USER_ID;

use super::constants::{DAYS_TO_EXPIRE, EXPIRE_AT};

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

pub fn create_token(user: &User) -> Result<String, String> {
    let mut claims = BTreeMap::new();
    claims.insert(USER_ID, user.id.to_string());
    let today = chrono::Utc::now().naive_utc();
    let expire_at = today + chrono::Duration::days(DAYS_TO_EXPIRE);
    claims.insert(EXPIRE_AT, expire_at.and_utc().timestamp().to_string());
    let key: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("SECRET_JWT_KEY")
            .expect("Secret key is not set")
            .as_bytes(),
    ).map_err(|e| e.to_string())?;
    claims.sign_with_key(&key).map_err(|e| e.to_string())
}
