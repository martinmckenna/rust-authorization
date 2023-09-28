use crate::utils::{jwt, AppState, BadPayload};
use actix_web::dev::ServiceRequest;
use actix_web::web;
use entity::blacklist::Entity as BlacklistEntity;
use entity::user::{Entity as UserEntity, TrimmedModel as TrimmedUserModel};
use sea_orm::{DatabaseBackend, DatabaseConnection, EntityTrait, Statement};
use std::sync::Mutex;

pub async fn authorize_user(
    req: ServiceRequest,
) -> Result<(TrimmedUserModel, String, ServiceRequest), (BadPayload, ServiceRequest)> {
    let authorization_header = req.headers().get("authorization");
    let app_data = req
        .app_data::<web::Data<Mutex<AppState>>>()
        .unwrap()
        .clone();
    let jwt_secret = app_data.lock().unwrap().jwt_secret.to_string();
    let conn = &app_data.lock().unwrap().connection;

    match authorization_header {
        Some(token) => match token.to_str() {
            Ok(token_as_string) => {
                let split_token = token_as_string.split_whitespace().collect::<Vec<_>>();

                if split_token.is_empty() || split_token[0] != "Bearer" || split_token.len() < 2 {
                    return Err((BadPayload {
                            error: "Please pass a valid authorization header beginning with \"Bearer\".".to_string(),
                            field: "header".to_string(),
                        }, req));
                }

                let maybe_token = BlacklistEntity::find()
                    .from_raw_sql(Statement::from_sql_and_values(
                        DatabaseBackend::Postgres,
                        r#"
                            SELECT *
                            FROM "blacklist"
                            WHERE "token" = $1
                        "#,
                        [split_token[1].into()],
                    ))
                    .one::<DatabaseConnection>(conn)
                    .await;

                if maybe_token.unwrap().is_some() {
                    return Err((
                        BadPayload {
                            error: "Token has already been revoked.".to_string(),
                            field: "token".to_string(),
                        },
                        req,
                    ));
                }

                match jwt::decode_token(split_token[1].to_string(), jwt_secret) {
                    Ok(jwt_payload) => {
                        match UserEntity::find()
                            .from_raw_sql(Statement::from_sql_and_values(
                                DatabaseBackend::Postgres,
                                r#"
                                    SELECT "user"."id", "user"."email", "user"."username" 
                                    FROM "user" 
                                    WHERE "id" = $1
                                "#,
                                [jwt_payload.claims.sub.into()],
                            ))
                            .into_model::<TrimmedUserModel>()
                            .one::<DatabaseConnection>(&conn)
                            .await
                        {
                            Ok(value) => Ok((value.unwrap(), split_token[1].to_string(), req)),
                            Err(_) => Err((
                                BadPayload {
                                    error: "Could not find user.".to_string(),
                                    field: "payload".to_string(),
                                },
                                req,
                            )),
                        }
                    }
                    Err(_) => Err((
                        BadPayload {
                            error: "Your token is either invalid or expired.".to_string(),
                            field: "header".to_string(),
                        },
                        req,
                    )),
                }
            }
            Err(_) => Err((
                BadPayload {
                    error: "Please pass a valid authorization header.".to_string(),
                    field: "header".to_string(),
                },
                req,
            )),
        },
        None => Err((
            BadPayload {
                error: "Please pass an authorization header.".to_string(),
                field: "header".to_string(),
            },
            req,
        )),
    }
}
