use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use cached::IOCachedAsync;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use serde_json;
use surrealdb::Datetime;

#[cfg(feature = "ssr")]
use surrealdb::{RecordId, Surreal, engine::any::Any};

#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
use crate::db_init;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<V> {
    pub id: RecordId,
    pub value: V,
    pub expires_at: Datetime, // Unix timestamp
}

#[cfg(feature = "ssr")]
#[derive(Error, Debug)]
pub enum SurrealCacheError {
    #[error("surrealdb error: {0}")]
    DatabaseError(#[from] surrealdb::Error),
    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("cache connection error")]
    ConnectionError,
}

#[cfg(feature = "ssr")]
pub struct AsyncSurrealCacheBuilder<K, V> {
    table_name: String,
    ttl: Option<Duration>,
    refresh: bool,
    _phantom: PhantomData<(K, V)>,
}

#[cfg(feature = "ssr")]
impl<K, V> AsyncSurrealCacheBuilder<K, V>
where
    K: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(table_name: impl Into<String>, ttl: Duration) -> Self {
        Self {
            table_name: table_name.into(),
            ttl: Some(ttl),
            refresh: false,
            _phantom: PhantomData,
        }
    }

    pub fn set_refresh(mut self, refresh: bool) -> Self {
        self.refresh = refresh;
        self
    }

    pub fn set_lifespan(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub async fn build(self) -> Result<AsyncSurrealCache<K, V>, SurrealCacheError> {
        let db = db_init()
            .await
            .map_err(|_| SurrealCacheError::ConnectionError)?;

        Ok(AsyncSurrealCache {
            db,
            table_name: self.table_name,
            ttl: self.ttl.unwrap_or(Duration::from_secs(3600)),
            refresh: self.refresh,
            _phantom: PhantomData,
        })
    }
}

#[cfg(feature = "ssr")]
pub struct AsyncSurrealCache<K, V> {
    db: Arc<Surreal<Any>>,
    table_name: String,
    ttl: Duration,
    refresh: bool,
    _phantom: PhantomData<(K, V)>,
}

#[cfg(feature = "ssr")]
impl<K, V> AsyncSurrealCache<K, V>
where
    K: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de>,
{
    #[allow(clippy::new_ret_no_self)]
    pub fn new(table_name: impl Into<String>, ttl: Duration) -> AsyncSurrealCacheBuilder<K, V> {
        AsyncSurrealCacheBuilder::new(table_name, ttl)
    }

    fn generate_key(&self, key: &K) -> Result<RecordId, SurrealCacheError> {
        let key_json = serde_json::to_string(key)?;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        key_json.hash(&mut hasher);
        let hash = hasher.finish();
        let record_id: RecordId = format!("{}:cache_{}", self.table_name, hash).parse()?;
        Ok(record_id)
    }

    fn get_current_timestamp() -> Datetime {
        Datetime::from(chrono::offset::Utc::now())
    }

    fn is_expired(entry: &CacheEntry<V>) -> bool {
        Self::get_current_timestamp().gt(&entry.expires_at)
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl<K, V> IOCachedAsync<K, V> for AsyncSurrealCache<K, V>
where
    K: Send + std::fmt::Debug + Sync + Clone + Serialize + for<'de> Deserialize<'de> + 'static,
    V: Send + Sync + Clone + Serialize + for<'de> Deserialize<'de> + 'static,
{
    type Error = SurrealCacheError;

    async fn cache_get(&self, key: &K) -> Result<Option<V>, Self::Error> {
        let key_id = self.generate_key(key)?;

        // Try to select the specific record
        let entry: Option<CacheEntry<V>> = self
            .db
            .select(key_id.clone())
            .await
            .map_err(SurrealCacheError::DatabaseError)?;

        if let Some(entry) = entry {
            if !Self::is_expired(&entry) {
                // Refresh TTL if enabled
                if self.refresh {
                    let expires_at: Datetime =
                        Datetime::from(chrono::offset::Utc::now() + self.ttl);

                    let updated_entry = CacheEntry {
                        id: entry.id.clone(),
                        value: entry.value.clone(),
                        expires_at,
                    };

                    self.db
                        .query("UPSERT $cacheidtorefresh CONTENT $content RETURN NONE;")
                        .bind(("cacheidtorefresh", key_id.clone()))
                        .bind(("content", updated_entry))
                        .await?;
                }
                return Ok(Some(entry.value));
            } else {
                // Entry expired, remove it
                let _: Option<CacheEntry<V>> = self
                    .db
                    .delete(key_id)
                    .await
                    .map_err(SurrealCacheError::DatabaseError)?;

                return Ok(None);
            }
        }

        Ok(None)
    }

    // asynchronously inserts a key-value pair into a cache and
    // returns the previously stored value associated with that key, or None if no value was present.
    async fn cache_set(&self, key: K, value: V) -> Result<Option<V>, Self::Error> {
        let key_id = self.generate_key(&key)?;
        let expires_at: Datetime = Datetime::from(chrono::offset::Utc::now() + self.ttl);

        let new_entry = CacheEntry {
            id: key_id.clone(),
            value: value.clone(),
            expires_at,
        };
        let mut query_res = self
            .db
            .query("UPSERT $cacheentryid CONTENT $content RETURN BEFORE;")
            .bind(("cacheentryid", key_id.clone()))
            .bind(("content", new_entry))
            .await?;
        let existing = query_res.take::<Option<CacheEntry<V>>>(0)?;

        Ok(existing.map(|e| e.value))
    }

    async fn cache_remove(&self, key: &K) -> Result<Option<V>, Self::Error> {
        let key_id = self.generate_key(key)?;

        let removed: Option<CacheEntry<V>> = self
            .db
            .delete(key_id)
            .await
            .map_err(SurrealCacheError::DatabaseError)?;

        Ok(removed.map(|entry| entry.value))
    }

    fn cache_set_refresh(&mut self, refresh: bool) -> bool {
        let old_refresh = self.refresh;
        self.refresh = refresh;
        old_refresh
    }

    fn cache_lifespan(&self) -> Option<Duration> {
        Some(self.ttl)
    }

    fn cache_set_lifespan(&mut self, ttl: Duration) -> Option<Duration> {
        let old_ttl = self.ttl;
        self.ttl = ttl;
        Some(old_ttl)
    }

    fn cache_unset_lifespan(&mut self) -> Option<Duration> {
        let old_ttl = self.ttl;
        self.ttl = Duration::from_secs(0); // Never expire
        Some(old_ttl)
    }
}

// AsyncSurrealCache is now fully implemented with IOCachedAsync trait
// and ready for use with #[io_cached] macro

// Test function using the cache
#[cfg(feature = "ssr")]
use cached::proc_macro::io_cached;
use std::thread::sleep;

#[cfg(feature = "ssr")]
#[io_cached(
    map_error = r##"|e| format!("Cache error: {:?}", e)"##,
    ty = "AsyncSurrealCache<String, u32>",
    create = r##" {
        AsyncSurrealCache::new("cache_table", Duration::from_secs(60))
            .set_refresh(true)
            .build()
            .await
            .expect("Failed to build SurrealDB cache")
    } "##,
    convert = r#"{ format!("{}-{}", a, b) }"#
)]
async fn slow_result(a: u32, b: u32) -> Result<u32, String> {
    sleep(Duration::new(2, 0));
    Ok(a * b)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct NestedData {
    baz: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct StructuredData {
    foo: u32,
    bar: f64,
    nested: NestedData,
}

#[cfg(feature = "ssr")]
#[io_cached(
    map_error = r##"|e| format!("Cache error: {:?}", e)"##,
    ty = "AsyncSurrealCache<String, StructuredData>",
    create = r##" {
        AsyncSurrealCache::new("cache_table", Duration::from_secs(60))
            .set_refresh(true)
            .build()
            .await
            .expect("Failed to build SurrealDB cache")
    } "##,
    convert = r#"{ format!("{}-{}", a, b) }"#
)]
async fn test_structured_data(a: u32, b: f64) -> Result<StructuredData, String> {
    sleep(Duration::new(2, 0));

    let output = StructuredData {
        foo: a,
        bar: b,
        nested: NestedData {
            baz: format!("{}", f64::from(a) * b),
        },
    };

    Ok(output)
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_slow_result() -> Result<(), Box<dyn std::error::Error>> {
    async fn some_test(input: u32) {
        let start = std::time::Instant::now();
        let first = slow_result(2, input).await;
        assert_eq!(first, Ok(input * 2));
        let diff = std::time::Instant::now() - start;
        println!("Time taken: {}ms", diff.as_millis());
    }

    some_test(1).await;
    some_test(2).await;
    some_test(3).await;

    async fn some_test_structured_data(input: f64) {
        let start = std::time::Instant::now();
        let first = test_structured_data(2, input).await;
        assert_eq!(
            first,
            Ok(StructuredData {
                foo: 2,
                bar: input,
                nested: NestedData {
                    baz: format!("{}", f64::from(2) * input),
                },
            })
        );
        let diff = std::time::Instant::now() - start;
        println!("Time taken: {}ms", diff.as_millis());
    }

    some_test_structured_data(1.0001).await;
    some_test_structured_data(2.0002).await;
    some_test_structured_data(3.00003).await;

    Ok(())
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_async_surreal_cache() -> Result<(), Box<dyn std::error::Error>> {
    use cached::IOCachedAsync;

    // Build the cache
    let cache = AsyncSurrealCache::<String, String>::new("test_cache", Duration::from_secs(30))
        .set_refresh(true)
        .build()
        .await?;

    // Test cache_set and cache_get
    let key = "test_key".to_string();
    let value = "test_value".to_string();

    // Set a value
    let old_value = cache.cache_set(key.clone(), value.clone()).await?;
    assert!(old_value.is_none());

    // Get the value back
    let cached_value = cache.cache_get(&key).await?;
    assert_eq!(cached_value, Some(value.clone()));

    // Test cache_remove
    let removed_value = cache.cache_remove(&key).await?;
    assert_eq!(removed_value, Some(value));

    // Verify it's gone
    let cached_value = cache.cache_get(&key).await?;
    assert!(cached_value.is_none());

    println!("AsyncSurrealCache test passed!");
    Ok(())
}
