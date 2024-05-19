use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct CheckResponse {
    status: String,
}

pub async fn health() -> impl Responder {
    let response = CheckResponse {
        status: "UP".to_string(),
    };

    HttpResponse::Ok().json(response)
}
