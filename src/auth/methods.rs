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

#[derive(Serialize)]
struct TokenResponse {
    token: String,
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

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().json(web::Json({}))
}

pub async fn register() -> HttpResponse {
    let register = LoginResponse {
        token: "1234".to_string(),
        username: "dummy-user".to_string(),
    };
    HttpResponse::Ok().json(web::Json(register))
}

pub async fn extend_token() -> HttpResponse {
    let token = TokenResponse {
        token: "1234".to_string(),
    };
    HttpResponse::Ok().json(web::Json(token))
}

pub async fn get_user() -> HttpResponse {
    let user = ProfileResponse {
        username: "another_user".to_string(),
    };
    HttpResponse::Ok().json(web::Json(user))
}
