use infrastructure::server;

mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return server::server::run().await;
}
