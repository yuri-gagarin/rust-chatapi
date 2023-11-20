/* JSON and JSON Validations */
use serde::{Deserialize, Serialize};
use validator::Validate;
//
use crate::MessageData;

#[derive(Clone, Serialize)]
pub struct ResponseData {
    pub response_message: String,
    pub message: Option<MessageData>,
    pub messages_list: Option<Vec<MessageData>>,
    pub new_message: Option<MessageData>,
    pub edited_message: Option<MessageData>,
    pub deleted_message: Option<MessageData>,
    pub total_messages: u32,
    pub error_messages: Option<Vec<String>>
}

#[derive(Serialize)]
pub struct CreateMsgRes {
    response_message: String,
    data: MessageData,
}

#[derive(Deserialize)]
pub struct MsgQueryParams {
    conversation_id: Option<String>,
    limit: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Validate)]
pub struct NewMsgReqData {
    #[validate(length(min = 3, message = "Must have a sender id"))]
    pub sender_id: Option<String>,
    #[validate(length(min = 3, message = "Must have a receiver id"))]
    pub receiver_id: Option<String>,
    #[validate(length(min = 3, message = "Must have a conversation id"))]
    pub conversation_id: Option<String>,
    #[validate(length(min = 3, message = "Must have a message content"))]
    pub content: Option<String>,
    #[validate(length(min = 3, message = "Must have a message topic"))]
    pub topic: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct EditMsgReqData {
    #[validate(length(min = 3, message = "Must have a sender id"))]
    pub sender_id: Option<String>,
    #[validate(length(min = 3, message = "Must have a receiver id"))]
    pub receiver_id: Option<String>,
    #[validate(length(min = 3, message = "Must have a conversation id"))]
    pub content: Option<String>,
}