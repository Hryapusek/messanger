use diesel::prelude::*;
use crate::schema::message_attachment;

#[derive(Queryable, Insertable)]
#[diesel(table_name = message_attachment)]
pub struct MessageAttachment {
    pub id: i32,
    pub message_id: i32,
    pub attachment_id: Option<i32>,
}
