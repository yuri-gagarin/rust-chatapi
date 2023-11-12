use actix_web::body::BoxBody;
use actix_web::{get, put, post, delete, web};
use actix_web::HttpResponse;
/* JSON and JSON Validations */
use validator::Validate;
/* Local dependencies */
use crate::{AppState, MessageData};
use crate::helpers::{data_helpers, validation_helpers};
use crate::helpers::response_helpers::MessageResponses;
//
use super::types::{NewMsgReqData, EditMsgReqData, MsgQueryParams, ResponseData};

// Message Model Routes //
#[get("/api/messages")]
pub async fn get_messages(data: web::Data<AppState>) -> HttpResponse {
    // println!("The state data is {:?}", data);
    let total = data.total_messages.lock().unwrap();
    let data_response = ResponseData {
        response_message: String::from("Current Messages"),
        messages: Some(data.messages.lock().unwrap().to_vec()),
        new_message: None,
        edited_message: None,
        deleted_message: None,
        total_messages: Some(*total),
        error_messages: None
    };
    HttpResponse::Ok().json(data_response)
}

#[post("/api/messages")]
pub async fn create_message(data: web::Data<AppState>, message_data: web::Json<NewMsgReqData>) -> HttpResponse<BoxBody> {
    // println!("The state data is {:?}", data);
    // println!("The new message data is {:?}", message_data);
    /* First validate the keys */ 
    let mut total_messages = data.total_messages.lock().unwrap();
    let (not_valid, error_messages) = validation_helpers::validate_new_message_input(&message_data);
    if not_valid {
        let msg = String::from("Invalid new message input");
        let data_response = MessageResponses::error_response(msg, &error_messages);
        return HttpResponse::BadRequest().json(data_response);
    }
    /* validate data */
    let validation_result = message_data.validate();
    if validation_result.is_err() {
        let msg = String::from("Invalid new message input");
        let errors = validation_result.unwrap_err();
        return HttpResponse::BadRequest().json(errors)
    }
    // mutable state //
    let mut message_state = data.messages.lock().unwrap();
    let NewMsgReqData { sender_id, receiver_id, content, .. } = message_data.0;
    let new_message = MessageData { 
      id: data_helpers::generate_rand_id(), 
      date: data_helpers::generate_date_now(), 
      sender: sender_id.unwrap(),
      receiver: receiver_id.unwrap(), 
      data: content.unwrap(), 
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
      total_messages: Some(*total_messages),
      error_messages: None
  };
    
    HttpResponse::Created().json(response)
}

#[put("/api/messages/{message_id}")]
pub async fn edit_message(data: web::Data<AppState>, message_data: web::Json<EditMsgReqData>,
                          path: web::Path<String>, query: web::Query<MsgQueryParams>) -> HttpResponse {
    /* Validate required JSON keys */
    let (invalid_data, error_messages) = validation_helpers::validate_edit_message_input(&message_data);
    if invalid_data {
        return HttpResponse::BadRequest().json(error_messages);
    }
    /* Validate correct data */
    let validation_result = message_data.validate();
    if validation_result.is_err() {
        let errors = validation_result.unwrap_err();
        return HttpResponse::BadRequest().json(errors)
    }
    /* ensure <message_id> is a valid int id */
    let (invalid_id_param, message_id) = data_helpers::invalid_id_params(path.into_inner());
    if invalid_id_param {
        let response_message: String = String::from("URL data error");
        let error_messages: Vec<String> = vec!["Could not resolve param <message_id>".into(), "Please check the correct URL".into()];
        let data_response: ResponseData = MessageResponses::error_response(response_message, &error_messages);
        return  HttpResponse::BadRequest().json(data_response);
    }
    let mut message_state = data.messages.lock().unwrap();
    let edit_index = message_state.iter().position(|val| val.id == message_id);
    if edit_index.is_none() {
        let response_message: String = String::from("Message edit error");
        let error_messages: Vec<String> = vec!["Could not resolve the message to edit".into(), "Please try again".into()];
        let data_response: ResponseData = MessageResponses::error_response(response_message, &error_messages);
        return  HttpResponse::BadRequest().json(data_response);
    }
    message_state[0].data = message_data.content.clone().unwrap();
    HttpResponse::Ok().json("ok")
}

#[delete("/api/messages/{message_id}")]
pub async fn delete_message(path: web::Path<String>) -> HttpResponse {
    let (message_id) = path.into_inner();
    HttpResponse::Ok().json("ok")
}
