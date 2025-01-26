// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Int4,
        #[sql_name = "type"]
        #[max_length = 30]
        type_ -> Varchar,
        document_id -> Nullable<Int4>,
    }
}

diesel::table! {
    direct_chat_message (id) {
        id -> Int4,
        direct_chat_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
    }
}

diesel::table! {
    direct_chats (id) {
        id -> Int4,
        user1_id -> Int4,
        user2_id -> Int4,
    }
}

diesel::table! {
    message_attachment (id) {
        id -> Int4,
        message_id -> Int4,
        attachment_id -> Nullable<Int4>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        text -> Nullable<Text>,
        created_at -> Timestamp,
        sender_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 30]
        surname -> Varchar,
        birth_date -> Nullable<Date>,
        #[max_length = 30]
        patronymic -> Nullable<Varchar>,
        #[max_length = 30]
        email -> Varchar,
        is_email_confirmed -> Nullable<Bool>,
        #[max_length = 30]
        phone -> Nullable<Varchar>,
        is_phone_confirmed -> Nullable<Bool>,
        #[max_length = 128]
        password -> Varchar,
    }
}

diesel::joinable!(direct_chat_message -> direct_chats (direct_chat_id));
diesel::joinable!(direct_chat_message -> messages (message_id));
diesel::joinable!(message_attachment -> attachments (attachment_id));
diesel::joinable!(message_attachment -> messages (message_id));
diesel::joinable!(messages -> users (sender_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    direct_chat_message,
    direct_chats,
    message_attachment,
    messages,
    users,
);
