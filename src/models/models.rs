use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    banned: bool,
    created_at: String,
    edited_at: String,
}

#[derive(Clone, Deserialize)]
pub struct UserConversations {
    user_id: String,
    conversation_id: String,
}

#[derive(Clone, Deserialize)]
pub struct Conversation {
    active: bool,
    hidden: bool,
    num_of_messages: i32,
    created_at: String,
    edited_at: String,
}

#[derive(Clone, Deserialize)]
pub struct Message {
    sender_id: String,
    receiver_id: String,
    conversation_id: String,
    content: String,
    topic: String,
    read: bool,
    created_at: String,
    edited_at: String,
}