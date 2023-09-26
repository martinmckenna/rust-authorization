use crate::utils::validation;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
// #[derive(Debug)]
pub struct Info {
    username: String,
    another: String,
}

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

#[derive(Serialize)]
struct Empty {}

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
    let empty = Empty {};
    HttpResponse::Ok().json(web::Json(empty))
}

pub async fn register(request_body: web::Bytes) -> HttpResponse {
    let register = LoginResponse {
        token: "1234".to_string(),
        username: "dummy-user".to_string(),
    };
    match validation::validate_json(&request_body, &vec!["username", "password"]) {
        Ok(_) => HttpResponse::Ok().json(web::Json(register)),
        Err(err) => HttpResponse::Ok().json(web::Json(err)),
    }
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
