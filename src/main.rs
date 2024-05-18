use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn first_endpoint() -> impl Responder {
    return "First endpoint!!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(first_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
