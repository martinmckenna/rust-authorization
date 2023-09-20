use actix_web::{web, App, HttpServer};
use rust_auth::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("").configure(auth::routes::routes)))
        .workers(4)
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
