use crate::auth::methods;
use actix_web::{web, Route};
use serde::Serialize;

#[derive(Serialize)]
struct ProfileResponse {
    username: String,
}

fn get_profile() -> Route {
    web::get().to(|| methods::get_profile())
}

fn login() -> Route {
    web::post().to(|| methods::login())
}

fn logout() -> Route {
    web::post().to(|| methods::logout())
}

fn register() -> Route {
    web::post().to(|| methods::register())
}

fn extend_token() -> Route {
    web::post().to(|| methods::extend_token())
}

fn get_user() -> Route {
    web::get().to(|| methods::get_user())
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/login", login())
            .route("/logout", logout())
            .route("/users/{name}", get_user())
            .route("/token/extend", extend_token())
            .route("/register", register())
            .route("/profile", get_profile()),
    );
}
