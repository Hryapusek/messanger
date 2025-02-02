use std::ops;

use crate::schema::users;
use crate::schema::users::*;

use diesel::prelude::*;
use crate::models::user::*;

use crate::connection::*;

pub fn create_user(user: User) -> Result<User, String>  {

  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  diesel::insert_into(users::table)
    .values(&user)
    .get_result(&mut conn)
    .map_err(|e| e.to_string())
}

pub fn get_user(obj_id: i32) -> Result<Option<User>, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  users::table
    .find(obj_id)
    .get_result(&mut conn)
    .optional()
    .map_err(|e| e.to_string())
}

pub fn get_user_by_email(user_email: &str) -> Result<Option<User>, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  users::table
    .filter(email.eq(user_email))
    .get_result(&mut conn)
    .optional()
    .map_err(|e| e.to_string())
}

pub fn get_user_by_phone(user_phone: String) -> Result<Option<User>, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  users::table
    .filter(phone.eq(user_phone))
    .get_result(&mut conn)
    .optional()
    .map_err(|e| e.to_string())
}

pub fn update_user(obj_id: i32, user: User) -> Result<Option<User>, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  diesel::update(users::table.find(obj_id))
    .set(&user)
    .get_result(&mut conn)
    .optional()
    .map_err(|e| e.to_string())
}

pub fn delete_user(obj_id: i32) -> Result<Option<User>, String> {
  let mut conn = establish_connection().map_err(|e| e.to_string())?;

  diesel::delete(users::table.find(obj_id))
    .get_result(&mut conn)
    .optional()
    .map_err(|e| e.to_string())
}
