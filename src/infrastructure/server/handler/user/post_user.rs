use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    email: String,
    password: String
}

pub async fn create_user(user: web::Json<User>) -> impl Responder {

    println!("User {}, Password {}", &user.email, &user.password);
    HttpResponse::Ok().json(user)
}
