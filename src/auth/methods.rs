use crate::utils::validation::{self, BadPayload};
use crate::utils::AppState;
use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use entity::user::{Entity as UserEntity, TrimmedModel as TrimmedUserModel};
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, Statement};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

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
    // token: String,
    username: String,
    id: i32,
    email: String,
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
        username: "dummy-user".to_string(),
        email: "dummy@email.com".to_string(),
        id: 12,
    };
    HttpResponse::Ok().json(web::Json(login))
}

pub async fn logout() -> HttpResponse {
    let empty = Empty {};
    HttpResponse::Ok().json(web::Json(empty))
}

pub async fn register(
    request_body: web::Bytes,
    app_state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    match validation::validate_json(&request_body, &vec!["username", "password", "email"]) {
        Ok(validated_request_body) => {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let password: &[u8] = match validated_request_body.get("password") {
                Some(value) => value.as_str().unwrap_or("").as_bytes(),
                None => "".as_bytes(),
            };
            let email = match validated_request_body.get("email") {
                Some(value) => value.as_str().unwrap_or("").to_string(),
                None => "".to_string(),
            };
            let username = match validated_request_body.get("username") {
                Some(value) => value.as_str().unwrap_or("").to_string(),
                None => "".to_string(),
            };

            let maybe_duplicate_user: Option<TrimmedUserModel> = match UserEntity::find()
                .from_raw_sql(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"
                        SELECT "user"."id", "user"."email", "user"."username" 
                        FROM "user" 
                        WHERE "username" = $1 OR "email" = $2
                    "#,
                    [username.clone().into(), email.clone().into()],
                ))
                .into_model::<TrimmedUserModel>()
                .one::<DatabaseConnection>(&app_state.lock().unwrap().connection)
                .await
            {
                Ok(value) => value,
                Err(_) => None,
            };

            if maybe_duplicate_user.is_some() {
                if maybe_duplicate_user.unwrap().email == email {
                    return HttpResponse::BadRequest().json(web::Json(BadPayload {
                        field: "email".to_string(),
                        error: "Email already exists".to_string(),
                    }));
                } else {
                    return HttpResponse::BadRequest().json(web::Json(BadPayload {
                        field: "username".to_string(),
                        error: "Username already exists".to_string(),
                    }));
                }
            }

            match password.is_empty() {
                true => HttpResponse::BadRequest().json(web::Json(validation::BadPayload {
                    error: "Please pass a valid password string".to_string(),
                    field: "password".to_string(),
                })),
                false => {
                    /*
                       we can now ensure the password is a valid string
                    */
                    let hashed_password = match argon2.hash_password(password, &salt) {
                        Ok(password_value) => password_value.to_string(),
                        Err(_) => "".to_string(),
                    };

                    let new_user = match app_state
                        .lock()
                        .unwrap()
                        .connection
                        .query_one(Statement::from_sql_and_values(
                            DatabaseBackend::Postgres,
                            r#"
                                INSERT INTO "user" (username, password, email)
                                VALUES ($1,$2,$3)
                                RETURNING *
                            "#,
                            [
                                username.clone().into(),
                                hashed_password.into(),
                                email.into(),
                            ],
                        ))
                        .await
                    {
                        Ok(value) => {
                            let value_copy = value.as_ref();
                            Some(LoginResponse {
                                username: value_copy
                                    .unwrap()
                                    .try_get("", "username")
                                    .unwrap_or("".to_string()),
                                id: value_copy.unwrap().try_get("", "id").unwrap_or(0),
                                email: value_copy
                                    .unwrap()
                                    .try_get("", "email")
                                    .unwrap_or("".to_string()),
                            })
                        }
                        Err(_) => None,
                    };

                    if new_user.is_some() {
                        let unwrapped_user = new_user.unwrap();

                        HttpResponse::Ok().json(web::Json(LoginResponse {
                            username: unwrapped_user.username,
                            email: unwrapped_user.email,
                            id: unwrapped_user.id,
                        }))
                    } else {
                        HttpResponse::BadRequest().json(web::Json(validation::BadPayload {
                            error: "Error creating user".to_string(),
                            field: "payload".to_string(),
                        }))
                    }
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
