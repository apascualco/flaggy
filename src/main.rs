use infrastructure::server;

mod infrastructure;
mod domain;
mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return server::server::run().await;
}
