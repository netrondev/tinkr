#[cfg(feature = "ssr")]
use chrono::Utc;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use std::fmt;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::any::Any};
#[cfg(feature = "ssr")]
use tracing::{Event, Subscriber};
#[cfg(feature = "ssr")]
use tracing_subscriber::layer::{Context, Layer};
#[cfg(feature = "ssr")]
use tracing_subscriber::registry::LookupSpan;
#[cfg(feature = "ssr")]
use uuid::Uuid;

use crate::{Datetime, RecordId};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogEvent {
    pub id: RecordId,
    pub timestamp: Datetime,
    pub level: String,
    pub target: String,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub fields: serde_json::Value,
}

#[cfg(feature = "ssr")]
pub struct DatabaseLayer {
    db: Arc<Surreal<Any>>,
}

#[cfg(feature = "ssr")]
impl DatabaseLayer {
    pub fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }

    async fn store_log(
        &self,
        log_event: LogEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _: Option<LogEvent> = self.db.create("log_events").content(log_event).await?;
        Ok(())
    }
}

#[cfg(feature = "ssr")]
impl<S> Layer<S> for DatabaseLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let dblogging: bool = "true"
            == std::env::var("DB_LOGGING")
                .unwrap_or_else(|_| "false".to_string())
                .to_lowercase();

        if !dblogging {
            return;
        }

        let metadata = event.metadata();

        // Extract fields from the event
        let mut field_visitor = JsonFieldVisitor::new();
        event.record(&mut field_visitor);

        let id = RecordId::from(("log_events", Uuid::new_v4().to_string()));

        let log_event = LogEvent {
            id,
            timestamp: Datetime::from(Utc::now()),
            level: metadata.level().to_string(),
            target: metadata.target().to_string(),
            message: field_visitor.message.unwrap_or_else(|| "".to_string()),
            module_path: metadata.module_path().map(|s| s.to_string()),
            file: metadata.file().map(|s| s.to_string()),
            line: metadata.line(),
            fields: field_visitor.fields,
        };

        //Store log event asynchronously
        let db = self.db.clone();
        tokio::spawn(async move {
            if let Err(e) = DatabaseLayer::new(db).store_log(log_event).await {
                eprintln!("Failed to store log event to database: {}", e);
            }
        });
    }
}

#[cfg(feature = "ssr")]
struct JsonFieldVisitor {
    message: Option<String>,
    fields: serde_json::Value,
}

#[cfg(feature = "ssr")]
impl JsonFieldVisitor {
    fn new() -> Self {
        Self {
            message: None,
            fields: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

#[cfg(feature = "ssr")]
impl tracing::field::Visit for JsonFieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{:?}", value));
        } else {
            if let serde_json::Value::Object(ref mut map) = self.fields {
                map.insert(
                    field.name().to_string(),
                    serde_json::Value::String(format!("{:?}", value)),
                );
            }
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_string());
        } else {
            if let serde_json::Value::Object(ref mut map) = self.fields {
                map.insert(
                    field.name().to_string(),
                    serde_json::Value::String(value.to_string()),
                );
            }
        }
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if let serde_json::Value::Object(ref mut map) = self.fields {
            map.insert(
                field.name().to_string(),
                serde_json::Value::Number(serde_json::Number::from(value)),
            );
        }
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        if let serde_json::Value::Object(ref mut map) = self.fields {
            map.insert(
                field.name().to_string(),
                serde_json::Value::Number(serde_json::Number::from(value)),
            );
        }
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if let serde_json::Value::Object(ref mut map) = self.fields {
            map.insert(field.name().to_string(), serde_json::Value::Bool(value));
        }
    }
}
