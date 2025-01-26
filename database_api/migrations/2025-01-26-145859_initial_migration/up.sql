-- Your SQL goes here
-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL,
    surname VARCHAR(30) NOT NULL,
    birth_date DATE,
    patronymic VARCHAR(30),
    email VARCHAR(30) NOT NULL UNIQUE,
    is_email_confirmed BOOLEAN DEFAULT FALSE,
    phone VARCHAR(30) UNIQUE,
    is_phone_confirmed BOOLEAN DEFAULT FALSE,
    password VARCHAR(128) NOT NULL
);

-- Create messages table
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    text TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    sender_id INTEGER NOT NULL REFERENCES users(id)
);

-- Create attachments table
CREATE TABLE attachments (
    id SERIAL PRIMARY KEY,
    type VARCHAR(30) NOT NULL,
    document_id INTEGER
);

-- Create message_attachment table
CREATE TABLE message_attachment (
    id SERIAL PRIMARY KEY,
    message_id INTEGER NOT NULL REFERENCES messages(id),
    attachment_id INTEGER REFERENCES attachments(id),
    UNIQUE (message_id, attachment_id)
);

-- Create direct_chats table
CREATE TABLE direct_chats (
    id SERIAL PRIMARY KEY,
    user1_id INTEGER NOT NULL REFERENCES users(id),
    user2_id INTEGER NOT NULL REFERENCES users(id),
    UNIQUE (user1_id, user2_id)
);

-- Create direct_chat_message table
CREATE TABLE direct_chat_message (
    id SERIAL PRIMARY KEY,
    direct_chat_id INTEGER REFERENCES direct_chats(id),
    message_id INTEGER REFERENCES messages(id),
    UNIQUE (direct_chat_id, message_id)
);
