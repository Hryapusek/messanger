use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub birth_date: Option<chrono::NaiveDate>,
    pub patronymic: Option<String>,
    pub email: String,
    pub is_email_confirmed: Option<bool>,
    pub phone: Option<String>,
    pub is_phone_confirmed: Option<bool>,
    pub password: String,
}
