pub mod connection;
pub mod settings;

#[cfg(feature = "ssr")]
pub use connection::{db_init, db_schema, db_seperate_connection};

#[cfg(feature = "ssr")]
pub use settings::{Settings, get_settings};

pub mod storage_trait;

#[cfg(feature = "ssr")]
pub use storage_trait::Storage;

#[cfg(feature = "ssr")]
pub mod cached_surrealdb;

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_database() -> Result<(), Box<dyn std::error::Error>> {
    use crate::db::connection::db_init;
    use crate::db::settings::get_settings;

    let settings = get_settings();
    println!("Settings: {:?}", settings);

    let db = db_init().await?;
    println!("Database connection successful: {:?}", db);

    Ok(())
}
