use display_json::DisplayAsJson;
use entity::user::TrimmedModel as TrimmedUser;
use sea_orm::DatabaseConnection;
use serde::Serialize;

pub mod authorize;
pub mod jwt;
pub mod validation;

#[derive(Clone, Serialize, Debug, DisplayAsJson)]
pub struct BadPayload {
    pub error: String,
    pub field: String,
}

#[derive(Debug)]
pub struct AppState {
    /* Mutex is necessary to mutate safely across threads */
    pub connection: DatabaseConnection,
    pub jwt_secret: String,
    pub user: Option<TrimmedUser>,
    pub jwt: Option<String>
}
