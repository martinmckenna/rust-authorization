use sea_orm::DatabaseConnection;

pub mod validation;

pub mod jwt;

pub struct AppState {
    /* Mutex is necessary to mutate safely across threads */
    pub connection: DatabaseConnection,
    pub jwt_secret: String
}