use actix_web::{App, HttpServer};
use rust_basic_actix_web::configure_app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(configure_app(None)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
