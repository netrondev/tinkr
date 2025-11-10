#[cfg(feature = "ssr")]
use crate::AppError;
#[cfg(feature = "ssr")]
use dotenvy::dotenv;
use std::env;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Settings {
    pub surrealdb_host: String,
    pub surrealdb_db: String,
    pub surrealdb_ns: String,
    pub surrealdb_user: String,
    pub surrealdb_pass: String,
}

#[cfg(feature = "ssr")]
pub fn get_env(key: &str) -> Result<String, AppError> {
    env::var(key)
        .map_err(|_| AppError::EnvVarError(format!("Environment variable {} is not set", key)))
}

#[cfg(feature = "ssr")]
pub fn get_settings() -> Settings {
    dotenv().ok();

    Settings {
        surrealdb_host: get_env("SURREALDB_HOST_NEW").unwrap_or("ws://localhost:8000".to_string()),
        surrealdb_db: get_env("SURREALDB_DB").unwrap_or("default".to_string()),
        surrealdb_ns: get_env("SURREALDB_NS").unwrap_or("default".to_string()),
        surrealdb_user: get_env("SURREALDB_USER").unwrap_or("root".to_string()),
        surrealdb_pass: get_env("SURREALDB_PASS").unwrap_or("root".to_string()),
    }
}
