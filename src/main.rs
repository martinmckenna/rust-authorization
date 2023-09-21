use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use rust_auth::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::scope("").configure(auth::routes::routes))
    })
    .workers(4)
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
