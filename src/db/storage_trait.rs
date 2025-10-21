#[cfg(feature = "ssr")]
use crate::AppError;
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[allow(async_fn_in_trait)]
pub trait Storage<NoId, WithId>
where
    NoId: Clone + Sync + Send + Serialize + 'static,
    WithId: Clone + Sync + Send + Serialize + for<'de> Deserialize<'de> + 'static,
{
    const TABLE_NAME: &str;

    /// Create or insert a new item
    // async fn create(content: NoId) -> Result<WithId, AppError>;

    async fn create(content: NoId) -> Result<WithId, AppError> {
        let db = crate::db_init().await?;

        let created_token = db.create(Self::TABLE_NAME).content(content).await?;

        match created_token {
            Some(token) => Ok(token),
            None => Err(AppError::GenericError("Failed to create token".into())),
        }
    }

    async fn get_many() -> Result<Vec<WithId>, AppError> {
        let db = crate::db_init().await?;

        let items: Vec<WithId> = db.select(Self::TABLE_NAME).await?;

        Ok(items)
    }

    async fn upsert(content: NoId, key: String) -> Result<WithId, AppError> {
        let db = crate::db_init().await?;

        let upserted_item = db.upsert((Self::TABLE_NAME, key)).content(content).await?;

        match upserted_item {
            Some(item) => Ok(item),
            None => Err(AppError::GenericError("Failed to upsert item".into())),
        }
    }

    async fn upsert_withid(content: WithId, key: String) -> Result<WithId, AppError> {
        let db = crate::db_init().await?;

        let upserted_item = db.upsert((Self::TABLE_NAME, key)).content(content).await?;

        match upserted_item {
            Some(item) => Ok(item),
            None => Err(AppError::GenericError("Failed to upsert item".into())),
        }
    }

    async fn upsert_many(content: Vec<(String, NoId)>) -> Result<Vec<WithId>, AppError> {
        let db = crate::db_init().await?;

        let mut results = Vec::new();
        for (key, item) in content {
            let upserted_item = db.upsert((Self::TABLE_NAME, key)).content(item).await?;
            match upserted_item {
                Some(item) => results.push(item),
                None => return Err(AppError::GenericError("Failed to upsert item".into())),
            }
        }

        Ok(results)
    }

    /*
       /// Read/get an item by key
       async fn from_id(&self, id: &RecordId) -> Result<Option<WithId>, E>;

       /// Update an existing item
       async fn update(&mut self, key: &NoId, value: WithId) -> Result<bool, E>;

       /// Delete an item by key
       async fn delete(&mut self, key: &NoId) -> Result<bool, E>;

       /// Check if a key exists
       async fn exists(&self, key: &NoId) -> Result<bool, E>;
       // fn exists(&self, key: &NoId) -> impl std::future::Future<Output = Result<bool, E>> + Send;

       /// Get all keys
       async fn keys(&self) -> Result<Vec<NoId>, E>;

       /// Get all values
       async fn values(&self) -> Result<Vec<WithId>, E>;

       /// Get all key-value pairs
       /// todo add filtering, sorting and pagination
       async fn entries(&self) -> Result<Vec<WithId>, E>;

       /// Clear all entries
       async fn clear(&mut self) -> Result<(), E>;

       /// Get the count of items
       async fn count(&self) -> Result<usize, E>;

       /// Batch create/insert multiple items
       /// Perhaps better to implement this without many create calls.
       async fn create_many(&mut self, items: Vec<NoId>) -> Result<Vec<WithId>, E> {
           let mut results = Vec::new();
           for value in items {
               let result = self.create(value).await?;
               results.push(result);
           }
           Ok(results)
       }

       /// Search with a predicate function
       async fn search<F>(&self, text: &str) -> Result<Vec<WithId>, E>;
    */
}
