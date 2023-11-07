use actix_web::{get, put, post, delete, web, App, http};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};
use crate::{AppState, MessageData};
use crate::helpers::data_helpers;
use std::collections::HashMap;
#[derive(Serialize)]
pub struct ResponseData {
    response_message: String,
    data: Vec<MessageData>,
}
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct NewMessageReqData {
    #[validate(length(min = 3, message = "Must have a sender id"))]
    sender_id: String,
    #[validate(length(min = 3, message = "Must have a receiver id"))]
    receiver_id: String,
    #[validate(length(min = 3, message = "Must have a conversation id"))]
    conversation_id: String,
    #[validate(length(min = 3, message = "Must have a message content"))]
    content: String,
    #[validate(length(min = 3, message = "Must have a message topic"))]
    topic: String,
}

// Message Model Routes //
#[get("/api/messages")]
pub async fn get_messages(data: web::Data<AppState>) -> HttpResponse {
    println!("The state data is {:?}", data);
    let data_response = ResponseData {
        response_message: String::from("Current Messages"),
        data: data.messages.lock().unwrap().to_vec(),
    };
    HttpResponse::Ok().json(data_response)
}

#[post("/api/messages")]
pub async fn create_message(data: web::Data<AppState>, message_data: web::Json<NewMessageReqData>) -> HttpResponse {
    //println!("The state data is {:?}", data);
    println!("The new message data is {:?}", message_data);
    let validation_result = message_data.validate();
    if validation_result.is_err() {
        let errors = validation_result.unwrap_err();
        return HttpResponse::BadRequest().json(errors)
    }
    /* 
    let sender_id = message_data.sender_id.clone().unwrap();
    let receiver_id = message_data.receiver_id.clone().unwrap();
    let content String = message_data.content.clone().unwrap();
    */
    let NewMessageReqData { sender_id, receiver_id, content, .. } = message_data.0;
    let new_message = MessageData { 
      id: data_helpers::generate_rand_id(), 
      date: data_helpers::generate_date_now(), 
      sender: sender_id,
      receiver: receiver_id, 
      data: content, 
      read: true,
    };


    HttpResponse::Created().json(new_message)
}

#[put("/api/messages/{message_id}")]
pub async fn edit_message(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json("ok")
}

#[delete("/api/messages/{message_id}")]
pub async fn delete_message(path: web::Path<String>) -> HttpResponse {
    let (message_id) = path.into_inner();
    HttpResponse::Ok().json("ok")
}
