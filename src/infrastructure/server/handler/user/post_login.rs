use std::sync::Arc;
use crate::{application::user::login_user::{Login, LoginService}, domain::user_repository::UserRepository, };

use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String
}

#[derive(Serialize)]
pub struct TokenResponse {
    token: String
}

pub async fn login_user<R: UserRepository>(
    user_case: web::Data<Arc<LoginService<R>>>,
    user: web::Json<LoginRequest>
) -> impl Responder {

    let token = user_case.login(&user.email, &user.password);
    match token {
        Ok(token) =>  HttpResponse::Ok().json(TokenResponse {token: token.build()}),
        Err(_) => HttpResponse::NotFound().finish()
    }
}
