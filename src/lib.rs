pub mod apperror;
pub mod builds;
pub mod date_utils;
pub mod status;

pub mod surrealtypes;
pub mod utils;

pub mod boring_avatars;

pub mod payments;

pub use status::Status;

#[cfg(feature = "ssr")]
pub use surrealdb::{Datetime, RecordId};

#[cfg(not(feature = "ssr"))]
pub use surrealtypes::{Datetime, RecordId};

pub mod arrays;
pub mod bytes;
pub mod colors;
pub mod components;
pub mod numbers;
pub mod strings;
pub mod theme;
pub mod urls;

#[cfg(feature = "ssr")]
pub mod db;

#[cfg(feature = "ssr")]
pub use db::cached_surrealdb;

#[cfg(feature = "ssr")]
pub use db::cached_surrealdb::AsyncSurrealCache;

#[cfg(feature = "ssr")]
pub use db::db_init;

#[cfg(feature = "ssr")]
pub use db::db_seperate_connection;

#[cfg(feature = "ssr")]
pub use db::storage_trait::Storage;

#[cfg(feature = "ssr")]
pub use db::connection::db_schema;

pub mod email;
pub use email::EmailAddress;

// AUTH

pub mod auth;
pub use auth::user;

#[cfg(feature = "ssr")]
pub use auth::adapter_rs_surreal;
pub use auth::callback;
pub use auth::session;
pub use auth::ui_auth;

#[cfg(feature = "ssr")]
pub use auth::token;

#[cfg(feature = "ssr")]
pub use auth::account;

pub use auth::authcheck;

pub use authcheck::AuthCheck;
pub mod storage_authed_trait;

#[cfg(feature = "ssr")]
pub use storage_authed_trait::StorageAuthed;

pub mod keys;
pub mod organization;
pub mod settings;
pub mod team;
pub mod users;
pub mod wallet;
pub use wallet::metamask;
pub mod admin;
pub mod logs;

pub use apperror::AppError;

#[cfg(feature = "ssr")]
pub mod telemetry;

#[cfg(feature = "ssr")]
pub mod middleware;

#[cfg(feature = "ssr")]
pub mod server;
