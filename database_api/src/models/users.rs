use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub birth_date: Option<chrono::NaiveDate>,
    pub patronymic: Option<String>,
    pub email: String,
    pub is_email_confirmed: bool,
    pub phone: Option<String>,
    pub is_phone_confirmed: bool,
    pub password: String,
}
