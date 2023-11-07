use actix_web::web;
use crate::routes::routes::NewMsgReqData;

pub fn validate_new_message_input(message_data: &web::Json<NewMsgReqData>) -> (bool, Vec<String>) {
    let mut errors: Vec<String> = Vec::new();
    let mut not_valid: bool = true;
    if message_data.sender_id.is_none() {
        errors.push(String::from("Field <sender_id> is missing"));
    }
    if message_data.receiver_id.is_none() {
        errors.push(String::from("Field <reciever_id> is missing"));
    }
    if message_data.conversation_id.is_none() {
        errors.push(String::from("Field <conversation_id"));
    }
    if message_data.content.is_none() {
        errors.push(String::from("Field <content> is missing"));
    }
    if message_data.topic.is_none() {
        errors.push(String::from("Field <topic> is missing"));
    }
    if errors.is_empty() {
       not_valid = false;
    }
    (not_valid, errors)
}

pub fn validate_edit_message_input(message_data: &web::Json<NewMsgReqData>) -> (bool, Vec<String>) {
    let (mut not_valid, mut errors) = (true, Vec::new());

    if message_data.sender_id.is_none() {
        errors.push(String::from("Field <sender_id> is missing"));
    }
    if message_data.content.is_none() {
        errors.push(String::from("Field <content> is missing"));
    }
    if errors.is_empty() {
        not_valid = false;
    }  
    (not_valid, errors)
}   