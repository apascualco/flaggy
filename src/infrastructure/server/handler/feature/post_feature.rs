use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

use crate::{application::feature::create_feature::{FeatureCreatorService, FeatureCreator}, domain::user_repository::UserRepository};

#[derive(Deserialize)]
pub struct FeatureRequest {
    #[allow(unused)]
    name: String
}

#[derive(Serialize)]
pub struct FeatureResponse {
    name: String,
}

pub async fn create_feature<R: UserRepository>(
    user_case: web::Data<Arc<FeatureCreatorService<R>>>,
    _user: web::Json<FeatureRequest>
) -> impl Responder {

    user_case.create_feature();
    HttpResponse::Ok().body("")
}
