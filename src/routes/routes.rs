use actix_web::{get, put, post, delete, web};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use crate::{AppState, MessageData};

#[derive(Deserialize)]
pub struct MessageReqData {

}

#[derive(Serialize)]
pub struct ResponseData {
    response_message: String,
    data: Vec<MessageData>,
}

// Message Model Routes //
#[get("/api/messages")]
pub async fn get_messages(data: web::Data<AppState>) -> HttpResponse {
    println!("The state data is {:?}", data);
    let messages = generate_mock_msg(5);
    let data_response = ResponseData {
        response_message: String::from("Current Messages"),
        data: messages
    };
    HttpResponse::Ok().json(data_response)
}

#[post("/api/messages")]
pub async fn create_message(message_data: web::Json<MessageReqData>) -> HttpResponse {
    HttpResponse::Created().json("ok")
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
