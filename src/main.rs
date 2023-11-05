use actix_web::{App, HttpResponse, HttpServer, Responder};
use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod models;
struct AppState {
    total_messages: u32,
    messages: Mutex<Vec<MessageData>>,
}

#[derive(Clone, Deserialize, Serialize)]
struct MessageData {
    id: i32,
    date: i64,
    sender: String,
    receiver: String,
    data: String,
    read: bool,
}

#[get("/")]
async fn index_route() -> impl Responder {
    let response = r#"{"index":0,"name":"AB/CDE/FG/402/test_int4","sts":"on","time":"2021-06-05 03:28:24.044284300 UTC","value":8}"#;
    HttpResponse::Ok().json(response)
}
#[post("/create")]
async fn create_route(request_body: String) -> impl Responder {
    HttpResponse::Ok().json(request_body)
}

async fn index_page() -> impl Responder {
    "Hello from index route"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState{
      total_messages: 0,
      messages: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index_route)
            .service(create_route)
            .service(web::scope("/app")).route("/index.html", web::get().to(index_page))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}