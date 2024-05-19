use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct CheckResponse {
    status: String,
}

#[get("/health")]
async fn health() -> HttpResponse {
    let response = CheckResponse {
        status: "UP".to_string(),
    };

    HttpResponse::Ok().json(response)
}
