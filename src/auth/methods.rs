use crate::utils::validation;
use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
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
        Ok(validated_request_body) => {
            // println!("{:?}", validated_request_body.get::<str>("password"));
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            // let password: &[u8];
            let password: &[u8] = match validated_request_body.get("password") {
                Some(value) => value.as_str().unwrap_or("").as_bytes(),
                None => "".as_bytes(),
            };

            match password.is_empty() {
                true => HttpResponse::BadRequest().json(web::Json(validation::BadPayload {
                    error: "Please pass a valid password string".to_string(),
                    field: "password".to_string(),
                })),
                false => {
                    /*
                       we can now ensure the password is a valid string
                    */
                    let password_hash = match argon2.hash_password(password, &salt) {
                        Ok(password_value) => password_value.to_string(),
                        Err(_) => "".to_string(),
                    };

                    println!("{:?}", password_hash);

                    HttpResponse::Ok().json(web::Json(register))
                }
            }
        }
        Err(err) => HttpResponse::BadRequest().json(web::Json(err)),
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
