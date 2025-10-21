#[cfg(feature = "ssr")]
use crate::AppError;

#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::user::AdapterUser;

#[cfg(feature = "ssr")]
use surrealdb::RecordId;

#[cfg(feature = "ssr")]
#[allow(async_fn_in_trait)]
pub trait StorageAuthed<NoId, WithId>
where
    Self: Clone + Serialize + for<'de> Deserialize<'de> + 'static,
    NoId: Clone + Sync + Send + Serialize + 'static,
    WithId: Clone + Sync + Send + Serialize + for<'de> Deserialize<'de> + 'static,
{
    const TABLE_NAME: &str;

    /// Create or insert a new item
    // async fn create(content: NoId) -> Result<WithId, AppError>;

    async fn create_by_user(user: AdapterUser, content: NoId) -> Result<WithId, AppError> {
        let db = crate::db_init().await?;

        let table = Self::TABLE_NAME;
        let query = format!(
            r#"
            LET $record = CREATE {table} CONTENT $content;
            UPDATE $record SET created_by_user_id = $user_id, created_at = time::now(), updated_at = time::now();
        "#
        );

        let mut result = db
            .query(query)
            .bind(("content", content))
            .bind(("user_id", user.id))
            .await?;

        let created_item: Option<WithId> = result.take(1)?;

        match created_item {
            Some(token) => Ok(token),
            None => Err(AppError::GenericError("Failed to create token".into())),
        }
    }

    async fn get_by_id(id: RecordId) -> Result<WithId, AppError> {
        let db = crate::db_init().await?;
        let item: Option<WithId> = db.select(id).await?;
        match item {
            Some(item) => Ok(item),
            None => Err(AppError::NotFound("Item not found".into())),
        }
    }

    async fn get_many() -> Result<Vec<WithId>, AppError> {
        let db = crate::db_init().await?;

        let items: Vec<WithId> = db.select(Self::TABLE_NAME).await?;

        Ok(items)
    }

    async fn get_by_user(user: AdapterUser) -> Result<Vec<WithId>, AppError> {
        use crate::db_init;
        let client = db_init().await?;
        let tablename = Self::TABLE_NAME;
        let query = format!(
            "SELECT * FROM {tablename} WHERE created_by_user_id = $user_id ORDER BY created_at DESC;"
        );

        let mut result = client.query(query).bind(("user_id", user.id)).await?;

        let items: Vec<WithId> = result.take(0)?;

        Ok(items)
    }

    async fn update(user: AdapterUser, _id: RecordId, content: WithId) -> Result<WithId, AppError> {
        // todo look up user permissions;

        let db = crate::db_init().await?;

        let query = format!(
            r#"
            LET $record = UPDATE $content.id MERGE $content;
            UPDATE $record SET created_by_user_id = $user_id, created_at = time::now(), updated_at = time::now();
            SELECT * from $record;
            "#
        );

        let mut response = db
            .query(query)
            .bind(("content", content))
            .bind(("user_id", user.id))
            .await?;

        let updated_item: Option<WithId> = response.take(2)?;

        match updated_item {
            Some(item) => Ok(item),
            None => Err(AppError::NotFound("Item not found".into())),
        }
    }

    async fn update_self(&self) -> Result<WithId, AppError> {
        let db = crate::db_init().await?;

        let mut query = db
            .query("UPDATE $content.id CONTENT $content")
            .bind(("content", self.clone()))
            .await?;

        let updated_item: Option<WithId> = query.take(0)?;

        match updated_item {
            Some(item) => Ok(item),
            None => Err(AppError::NotFound("Item not found".into())),
        }
    }

    async fn delete(_user: AdapterUser, id: RecordId) -> Result<bool, AppError> {
        // todo look up user permissions;

        let db = crate::db_init().await?;
        let deleted: Option<WithId> = db.delete(id).await?;

        match deleted {
            Some(_) => Ok(true),
            None => Err(AppError::NotFound("Item not found".into())),
        }
    }

    async fn delete_self(&self) -> Result<bool, AppError> {
        let db = crate::db_init().await?;

        let mut query = db
            .query("DELETE $content.id;")
            .bind(("content", self.clone()))
            .await?;

        let deleted: Option<WithId> = query.take(0)?;

        match deleted {
            Some(_) => Ok(true),
            None => Err(AppError::NotFound("Item not found".into())),
        }
    }
}
