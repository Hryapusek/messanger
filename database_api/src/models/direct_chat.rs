use diesel::prelude::*;
use crate::schema::direct_chats;

#[derive(Queryable, Insertable)]
#[diesel(table_name = direct_chats)]
pub struct DirectChat {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
}
