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

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("login", login())
            .route("profile", get_profile()),
    );
}
