#[cfg(feature = "ssr")]
use crate::AppError;
#[cfg(feature = "ssr")]
use crate::db::settings;

#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use std::time::Duration;
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::any::Any, opt::auth::Root};

#[cfg(feature = "ssr")]
use tokio::sync::OnceCell;

#[cfg(feature = "ssr")]
use tokio::time::{sleep, timeout};

#[cfg(feature = "ssr")]
static DB: OnceCell<Arc<Surreal<Any>>> = OnceCell::const_new();

#[cfg(feature = "ssr")]
pub async fn db_init() -> Result<Arc<Surreal<Any>>, AppError> {
    let mut db = DB
        .get_or_try_init(|| async {
            let settings = settings::get_settings();

            const MAX_RETRIES: u32 = 5;
            const INITIAL_DELAY: Duration = Duration::from_millis(500);
            const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

            for attempt in 1..=MAX_RETRIES {
                let connect_result = timeout(CONNECTION_TIMEOUT, async {
                    let db = surrealdb::engine::any::connect(&settings.surrealdb_host).await?;

                    db.signin(Root {
                        username: &settings.surrealdb_user,
                        password: &settings.surrealdb_pass,
                    })
                    .await?;
                    db.use_ns(&settings.surrealdb_ns).await?;
                    db.use_db(&settings.surrealdb_db).await?;
                    Ok::<_, AppError>(Arc::new(db))
                })
                .await;

                match connect_result {
                    Ok(Ok(db)) => return Ok(db),
                    Ok(Err(e)) => {
                        if attempt == MAX_RETRIES {
                            return Err(e);
                        }
                        let delay = INITIAL_DELAY * 2_u32.pow(attempt - 1);
                        sleep(delay).await;
                    }
                    Err(_) => {
                        if attempt == MAX_RETRIES {
                            return Err(AppError::DatabaseError(
                                "Database connection timeout".into(),
                            ));
                        }
                        let delay = INITIAL_DELAY * 2_u32.pow(attempt - 1);
                        sleep(delay).await;
                    }
                }
            }

            unreachable!()
        })
        .await
        .cloned();

    db = match db {
        Ok(db) => {
            let settings = settings::get_settings();
            db.use_ns(&settings.surrealdb_ns).await?;
            db.use_db(&settings.surrealdb_db).await?;

            Ok(db)
        }
        Err(e) => {
            return Err(AppError::DatabaseError(format!(
                "Failed to connect to database: {}",
                e
            )));
        }
    };

    db
}

// used for background tests to have a separate connection
#[cfg(feature = "ssr")]
pub async fn db_seperate_connection() -> Result<Surreal<Any>, AppError> {
    let settings = settings::get_settings();

    let db = surrealdb::engine::any::connect(&settings.surrealdb_host).await?;

    db.signin(Root {
        username: &settings.surrealdb_user,
        password: &settings.surrealdb_pass,
    })
    .await?;

    db.use_ns(&settings.surrealdb_ns).await?;
    db.use_db(&settings.surrealdb_db).await?;

    Ok(db)
}

#[cfg(feature = "ssr")]
pub async fn db_schema() -> Result<(), AppError> {
    let db = db_init().await?;

    let schema = r#"
        remove field if exists email on table user;
        REMOVE INDEX if exists user_email_index ON TABLE user;
        update token set mc = 0 where mc > 1_000_000_000_000_000;
        update token set last_trade = time::from::unix(last_trade_unix_time/1000);
        
        -- Log events table for tracing
        DEFINE TABLE log_events SCHEMAFULL;
        DEFINE FIELD id ON TABLE log_events TYPE string;
        DEFINE FIELD timestamp ON TABLE log_events TYPE datetime;
        DEFINE FIELD level ON TABLE log_events TYPE string;
        DEFINE FIELD target ON TABLE log_events TYPE string;
        DEFINE FIELD message ON TABLE log_events TYPE string;
        DEFINE FIELD module_path ON TABLE log_events TYPE option<string>;
        DEFINE FIELD file ON TABLE log_events TYPE option<string>;
        DEFINE FIELD line ON TABLE log_events TYPE option<number>;
        DEFINE FIELD fields ON TABLE log_events TYPE object;
        
        -- Index for querying logs by timestamp and level
        DEFINE INDEX log_events_timestamp ON TABLE log_events COLUMNS timestamp;
        DEFINE INDEX log_events_level ON TABLE log_events COLUMNS level;
        DEFINE INDEX log_events_target ON TABLE log_events COLUMNS target;

       
    "#;

    let _ = db.query(schema).await;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn db_health() -> Result<bool, AppError> {
    let db = db_seperate_connection().await?;
    let _ = db.version().await?;
    Ok(true)
}
