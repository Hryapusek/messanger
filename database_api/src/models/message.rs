use diesel::prelude::*;
use crate::schema::messages;

#[derive(Queryable, Insertable)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,
    pub text: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub sender_id: i32,
}
