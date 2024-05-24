use std::sync::Arc;
use crate::application::user::create_user::UserCreator;

use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserRequest {
    email: String,
    password: String
}

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    email: String
}

pub async fn create_user(
    user_case: web::Data<Arc<dyn UserCreator>>,
    user: web::Json<UserRequest>
) -> impl Responder {

    user_case.create_user(&user.email, &user.password);
    //HttpResponse::Ok().json(user)
    HttpResponse::Ok().body("OK")
}
