use actix_web::body::BoxBody;
use actix_web::{get, put, post, delete, web};
use actix_web::HttpResponse;
/* JSON and JSON Validations */
use validator::Validate;
/* Local dependencies */
use crate::AppState;
use crate::helpers::{data_helpers, validation_helpers};
use crate::helpers::response_helpers::{MessageResponses, SingleMsgRes};
//
use super::types::{NewMsgReqData, EditMsgReqData, MsgQueryParams, ResponseData};

// Message Model Routes //
#[get("/api/messages")]
pub async fn get_messages(data: web::Data<AppState>) -> HttpResponse {
    // println!("The state data is {:?}", data);
    let total_messages = data.total_messages.lock().unwrap();
    let messages = data.messages.lock().unwrap();
    let data_response = MessageResponses::all_messages_response(
        String::from("All messages"),
        *total_messages,
        messages.to_vec()
    );
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
        let data_response = MessageResponses::error_response(
            String::from("Invalid new message input"),
            *total_messages,
            error_messages
        );
        return HttpResponse::BadRequest().json(data_response);
    }
    /* validate data */
    let validation_result = message_data.validate();
    if validation_result.is_err() {
        //let msg = String::from("Invalid new message input");
        let errors = validation_result.unwrap_err();
        return HttpResponse::BadRequest().json(errors);
    }
    // mutable state //
    let mut message_state = data.messages.lock().unwrap();
    let NewMsgReqData { sender_id, receiver_id, content, .. } = message_data.0;

    let msg_id: i64 = 1000 + (*total_messages - 1) as i64;
    let new_message = data_helpers::create_new_message(
        msg_id, 
        sender_id.unwrap(), 
        receiver_id.unwrap(), 
        content.unwrap()
    );
    message_state.push(new_message.clone());
    *total_messages += 1;
    let response_data = MessageResponses::single_message_response(
      String::from("New message created"), 
      *total_messages, 
      new_message, 
      SingleMsgRes::CreatedRes
    );
    
    HttpResponse::Created().json(response_data)
}

#[put("/api/messages/{message_id}")]
pub async fn edit_message(data: web::Data<AppState>, message_data: web::Json<EditMsgReqData>,
                          path: web::Path<String>, _query: web::Query<MsgQueryParams>) -> HttpResponse {
    let total_messages = *data.total_messages.lock().unwrap();
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
        let data_response: ResponseData = MessageResponses::error_response(response_message, total_messages, error_messages);
        return  HttpResponse::BadRequest().json(data_response);
    }
    let mut message_state = data.messages.lock().unwrap();
    let edit_index = message_state.iter().position(|val| val.id == message_id);
    if edit_index.is_none() {
        let response_message: String = String::from("Message edit error");
        let error_messages: Vec<String> = vec!["Could not resolve the message to edit".into(), "Please try again".into()];
        let data_response: ResponseData = MessageResponses::error_response(response_message, total_messages, error_messages);
        return  HttpResponse::BadRequest().json(data_response);
    }

    let i = edit_index.unwrap();
    message_state[i].data = message_data.content.clone().unwrap();
    // all ok response //
    let response_message = String::from("Message edited");
    let response_data = MessageResponses::single_message_response(
      response_message, total_messages, message_state[i].clone(), SingleMsgRes::EditedRes);

    HttpResponse::Ok().json(response_data)
}

#[delete("/api/messages/{message_id}")]
pub async fn delete_message(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let message_id = path.into_inner();
    let mut total_messages = data.total_messages.lock().unwrap();
    /* ensure <message_id> is a valid int id */
    let (invalid_id_param, msg_id) = data_helpers::invalid_id_params(message_id);
    if invalid_id_param {
        let response_message: String = String::from("URL data error");
        let error_messages: Vec<String> = vec!["Could not resolve param <message_id>".into(), "Please check the correct URL".into()];
        let data_response: ResponseData = MessageResponses::error_response(response_message, *total_messages, error_messages);
        return  HttpResponse::BadRequest().json(data_response);
    }
    /* Find the index and remove */
    let mut message_state = data.messages.lock().unwrap();
    let remove_index = message_state.iter().position(|val| val.id == msg_id);
    if remove_index.is_none() {
        let response_message: String = String::from("Message delete error");
        let error_messages: Vec<String> = vec!["Could not resolve the message to delete".into(), "Please try again".into()];
        let data_response: ResponseData = MessageResponses::error_response(response_message, *total_messages, error_messages);
        return  HttpResponse::BadRequest().json(data_response);
    }
    let removed_msg = message_state.remove(remove_index.unwrap());
    *total_messages -= 1;
    let response_data = MessageResponses::single_message_response(
        String::from("Message deleted"), 
        *total_messages,
        removed_msg,
        SingleMsgRes::DeletedRes
    );
    HttpResponse::Ok().json(response_data)
}
