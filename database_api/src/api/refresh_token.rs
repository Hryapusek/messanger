use std::ops;

use crate::models::refresh_token::RefreshToken;
use crate::schema::refresh_tokens;

use diesel::prelude::*;

use crate::connection::*;

pub fn create_refresh_token(refresh_token: RefreshToken) -> Result<RefreshToken, String>  {

  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  diesel::insert_into(refresh_tokens::table)
    .values(&refresh_token)
    .get_result(&mut conn)
    .map_err(|e| e.to_string())
}

pub fn get_refresh_token(obj_id: i32) -> Result<RefreshToken, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  refresh_tokens::table
    .find(obj_id)
    .get_result(&mut conn)
    .map_err(|e| e.to_string())
}

pub fn update_refresh_token(obj_id: i32, user: RefreshToken) -> Result<RefreshToken, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  diesel::update(refresh_tokens::table.find(obj_id))
    .set(&user)
    .get_result(&mut conn)
    .map_err(|e| e.to_string())
}

pub fn delete_refresh_token(obj_id: i32) -> Result<RefreshToken, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  diesel::delete(refresh_tokens::table.find(obj_id))
    .get_result(&mut conn)
    .map_err(|e| e.to_string())
}
