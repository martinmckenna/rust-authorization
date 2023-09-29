use std::sync::Mutex;

use actix_cors::Cors;
use actix_http::body::BoxBody;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::Error,
    http, web, App, HttpServer,
};
use actix_web_lab::middleware::{from_fn, Next};
use dotenvy;
use migration::{Migrator, MigratorTrait};
use rust_auth::auth;
use rust_auth::utils::authorize::authorize_user;
use rust_auth::utils::AppState;
use sea_orm::Database;

async fn auth_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_routes: Vec<&str> = vec!["/profile", "/logout"];

    /*
       try and authenticate the user for routes that require an
       auth token
    */
    if auth_routes.iter().any(|e| req.path().contains(e)) {
        match authorize_user(req).await {
            Ok((authed_user, jwt, proxy_request)) => {
                proxy_request
                    .app_data::<web::Data<Mutex<AppState>>>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .user = Some(authed_user);
                proxy_request
                    .app_data::<web::Data<Mutex<AppState>>>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .jwt = Some(jwt);
                next.call(proxy_request).await
            }
            Err((error, proxy_request)) => {
                let unauth_error = actix_web::error::ErrorUnauthorized(web::Json(error));
                Ok(proxy_request.error_response(unauth_error))
            }
        }
    } else {
        next.call(req).await
    }
}

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
        user: None,
        jwt: None,
    }));
    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(auth_middleware))
            .wrap(
                Cors::default()
                    .send_wildcard()
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
