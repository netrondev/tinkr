use chrono::Utc;
use crate::EmailAddress;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::db_init;

use crate::{AppError, date_utils::parse_surrealdb_datetime_to_chrono};

#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateVerificationToken",
    derive(Serialize, Deserialize),
    omit(id, token, expires)
)]
#[partial(
    "CreateVerificationTokenConent",
    derive(Serialize, Deserialize),
    omit(id)
)]
pub struct VerificationToken {
    pub id: RecordId,
    pub email: EmailAddress,
    pub user_id: RecordId,
    pub expires: Datetime,
    pub token: String,
}

impl VerificationToken {
    pub async fn create_verification_token(
        content: CreateVerificationToken,
    ) -> Result<VerificationToken, AppError> {
        let client = db_init().await?;

        let newtoken_string = Uuid::new_v4().to_string();

        let mut query = client.query("CREATE verificationToken SET email = $email, expires = time::now() + 30m, token = $tokenstring, user_id = $user_id;")
            .bind(("email", content.email))
            .bind(("tokenstring", newtoken_string))
            .bind(("user_id", content.user_id))
            .await?;

        let result: Option<Self> = query.take(0)?;

        match result {
            Some(token) => Ok(token),
            None => Err(AppError::AuthError(
                "Could not create verification token".into(),
            )),
        }
    }

    pub async fn use_verification_token(token: String) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query("DELETE verificationToken WHERE token = $tokenstring RETURN BEFORE;")
            .bind(("tokenstring", token))
            .await?;

        let token: Option<Self> = result.take(0)?;

        tracing::info!("use_verification_token result: {:#?}", token);

        match token {
            Some(token) => {
                let expired = parse_surrealdb_datetime_to_chrono(&token.expires)
                    .ok_or_else(|| AppError::AuthError("Invalid token date".into()))?;

                if expired < Utc::now() {
                    return Err(AppError::AuthError("Token has expired".into()));
                }
                Ok(token)
            }
            None => return Err(AppError::AuthError("Token not found".into())),
        }
    }
}
