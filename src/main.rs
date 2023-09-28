use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenvy;
use migration::{Migrator, MigratorTrait};
use rust_auth::auth;
use rust_auth::utils::AppState;
use sea_orm::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect(".env file not found");
    let db_user = dotenvy::var("POSTGRES_USER").expect("POSTGRES_USER is not set in .env file");
    let db_pass =
        dotenvy::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD is not set in .env file");
    let db_name = dotenvy::var("POSTGRES_DB").expect("POSTGRES_DB is not set in .env file");
    let host = dotenvy::var("POSTGRES_HOST").expect("POSTGRES_HOST is not set in .env file");
    let jwt_secret =
        dotenvy::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY is not set in .env file");
    let server_url = format!("postgres://{db_user}:{db_pass}@{host}:5432/{db_name}");

    let conn = Database::connect(&server_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let app_state = web::Data::new(Mutex::new(AppState {
        connection: conn,
        jwt_secret: jwt_secret,
    }));
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(app_state.clone())
            .service(web::scope("").configure(auth::routes::routes))
    })
    .workers(4)
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
