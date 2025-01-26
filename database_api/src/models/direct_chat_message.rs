use diesel::prelude::*;
use crate::schema::direct_chat_message;

#[derive(Queryable, Insertable)]
#[diesel(table_name = direct_chat_message)]
pub struct DirectChatMessage {
    pub id: i32,
    pub direct_chat_id: Option<i32>,
    pub message_id: Option<i32>,
}
