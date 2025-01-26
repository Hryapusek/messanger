use diesel::prelude::*;
use crate::schema::attachments;

#[derive(Queryable, Insertable)]
#[diesel(table_name = attachments)]
pub struct Attachment {
    pub id: i32,
    pub type_: String, // Renamed because `type` is a reserved keyword
    pub document_id: Option<i32>,
}
