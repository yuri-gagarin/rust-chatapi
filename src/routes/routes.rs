use std::ops::Add;

use actix_web::body::BoxBody;
use actix_web::{get, put, post, delete, web, App, http};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};
use crate::{AppState, MessageData};
use crate::helpers::data_helpers;
#[derive(Serialize)]
pub struct ResponseData {
    response_message: String,
    messages: Option<Vec<MessageData>>,
    new_message: Option<MessageData>,
    edited_message: Option<MessageData>,
    deleted_message: Option<MessageData>,
    total_messages: u32,
}
#[derive(Serialize)]
pub struct CreateMsgRes {
    response_message: String,
    data: MessageData,
    something: Option<String>
}
#[derive(Deserialize)]
pub struct MsgQueryParams {
    conversation_id: Option<String>,
    limit: Option<String>,
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
    let total = data.total_messages.lock().unwrap();
    let data_response = ResponseData {
        response_message: String::from("Current Messages"),
        messages: Some(data.messages.lock().unwrap().to_vec()),
        new_message: None,
        edited_message: None,
        deleted_message: None,
        total_messages: *total,
    };
    HttpResponse::Ok().json(data_response)
}

#[post("/api/messages")]
pub async fn create_message(data: web::Data<AppState>, message_data: web::Json<NewMessageReqData>) -> HttpResponse<BoxBody> {
    // println!("The state data is {:?}", data);
    // println!("The new message data is {:?}", message_data);
    let validation_result = message_data.validate();
    if validation_result.is_err() {
        let errors = validation_result.unwrap_err();
        return HttpResponse::BadRequest().json(errors)
    }
    // mutable state //
    let mut message_state = data.messages.lock().unwrap();
    let mut total_messages = data.total_messages.lock().unwrap();
    let NewMessageReqData { sender_id, receiver_id, content, .. } = message_data.0;
    let new_message = MessageData { 
      id: data_helpers::generate_rand_id(), 
      date: data_helpers::generate_date_now(), 
      sender: sender_id,
      receiver: receiver_id, 
      data: content, 
      read: true,
    };

    message_state.push(new_message.clone());
    *total_messages += 1;


    // response //
    let response = ResponseData {
      response_message: "A new message created".to_string(),
      new_message: Some(new_message),
      messages: None,
      edited_message: None,
      deleted_message: None,
      total_messages: *total_messages
  };
    
    HttpResponse::Created().json(response)
}

#[put("/api/messages/{message_id}")]
pub async fn edit_message(data: web::Data<AppState>, path: web::Path<String>, query: web::Query<MsgQueryParams>) -> HttpResponse {
    if query.conversation_id.is_none(){
        println!("Conversation id does ot exist");
    }
    let (message_id) = path.into_inner();
    println!("The path is {}", message_id);
    HttpResponse::Ok().json("ok")
}

#[delete("/api/messages/{message_id}")]
pub async fn delete_message(path: web::Path<String>) -> HttpResponse {
    let (message_id) = path.into_inner();
    HttpResponse::Ok().json("ok")
}
