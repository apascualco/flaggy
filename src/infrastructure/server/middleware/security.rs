use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::domain::token::TokenClaims;


pub struct Security {
    pub min_role: String
}

impl<S, B> Transform<S, ServiceRequest> for Security
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityMiddleware {
            service: Rc::new(service),
            min_role: self.min_role.clone()
        })
    }
}

pub struct SecurityMiddleware<S> {
    service: Rc<S>,
    min_role: String
}

impl<S, B> Service<ServiceRequest> for SecurityMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let authorization = req
            .headers()
            .get("Authorization")
            .and_then(|header_value| header_value.to_str().ok())
            .map(|header_str| header_str.to_string());

        let token = match authorization {
            Some(bearer) => {
                match bearer.strip_prefix("Bearer ") {
                    Some(auth_token) => auth_token.to_string(),
                    None => "".to_string()
                }
            },
            None => "".to_string()
        };

        if token.is_empty() {
            return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) });
        }

        match  TokenClaims::new(token) {
            Ok(claims) => {
                // Add redis and check the token if exist
                // Check user [active, valid, etc]
                if claims.get_roles().contains(&self.min_role) || 
                    claims.get_roles().contains(&"ADMIN".to_string()){
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) });
            },
            Err(_) => Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) })
        }
    }

}
