use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ProfileResponse {
    username: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    username: String,
}

pub async fn get_profile() -> HttpResponse {
    let profile = ProfileResponse {
        username: "dummy-user".to_string(),
    };
    HttpResponse::Ok().json(web::Json(profile))
}

pub async fn login() -> HttpResponse {
    let login = LoginResponse {
        token: "1234".to_string(),
        username: "dummy-user".to_string(),
    };
    HttpResponse::Ok().json(web::Json(login))
}
