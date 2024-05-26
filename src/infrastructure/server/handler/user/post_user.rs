use std::sync::Arc;
use crate::{application::user::create_user::{UserCreator, UserCreatorService}, domain::user_repository::UserRepository, };

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

pub async fn create_user<R: UserRepository>(
    user_case: web::Data<Arc<UserCreatorService<R>>>,
    user: web::Json<UserRequest>
) -> impl Responder {

    let new_user = user_case.create_user(&user.email, &user.password);
    let response = UserResponse {
        id: new_user.get_id(),
        email: new_user.get_email().to_string(),
    };
    HttpResponse::Ok().json(response)
}
