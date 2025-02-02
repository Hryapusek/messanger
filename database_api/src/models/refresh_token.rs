use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}
