#[cfg(feature = "ssr")]
use crate::email::address::EmailAddress;

#[cfg(feature = "ssr")]
use crate::AppError;

#[cfg(feature = "ssr")]
use crate::db_init;

use partial_struct::Partial;

#[cfg(feature = "ssr")]
use resend_rs::{
    Resend,
    types::{CreateEmailBaseOptions, CreateEmailResponse},
};

use serde::{Deserialize, Serialize};

use crate::{Datetime, RecordId};

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct ResendClient {
    pub client: Resend,
    pub from_email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailResponse {
    pub id: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub created_at: String,
}

/// Store the record of email sending in the db
#[derive(Serialize, Debug, Deserialize, Clone, Partial)]
#[partial(
    "EmailResultDBNewRow",
    derive(Serialize, Debug, Deserialize, Clone),
    omit(id)
)]
pub struct EmailResultDB {
    pub id: RecordId,
    pub message: String,
    pub email: String,
    pub subject: String,
    pub send_requested: bool,
    pub sent_at: Datetime,
    pub video: Option<RecordId>,
    pub user_id: Option<RecordId>,
    pub email_response: Option<EmailResponse>,
}

#[cfg(feature = "ssr")]
impl EmailResultDB {
    pub async fn save_to_db(content: EmailResultDBNewRow) -> Result<Self, AppError> {
        let db = db_init().await?;
        let recorded: Option<Self> = db.create("email_result").content(content).await?;

        match recorded {
            Some(rec) => Ok(rec),
            None => Err("could not record email entry".into()),
        }
    }

    pub async fn list() -> Result<Vec<Self>, AppError> {
        let db = db_init().await?;
        let res: Vec<Self> = db.select("email_result").await?;
        println!("{:#?}", res);
        Ok(res)
    }
}

#[cfg(feature = "ssr")]
#[tokio::test]
#[ignore = "do not run this test in production"]
async fn test_email_result_db_list() -> Result<(), AppError> {
    let _ = EmailResultDB::list().await?;
    Ok(())
}

#[cfg(feature = "ssr")]
impl ResendClient {
    pub fn try_init() -> Result<ResendClient, AppError> {
        dotenvy::dotenv().ok();

        if std::env::var("RESEND_FROM").is_err() {
            tracing::warn!(
                "RESEND_FROM environment variable is not set. Skipping Resend client initialization."
            );
            return Err("RESEND_FROM environment variable is not set".into());
        }

        let api_key = std::env::var("RESEND_API_KEY")?;
        let from_email = std::env::var("RESEND_FROM")?;
        let client = Resend::new(&api_key);
        println!("Resend client initialized");
        let success = Self { client, from_email };
        Ok(success)
    }

    pub async fn send_email(
        &self,
        to: EmailAddress,
        subject: &str,
        body: &str,
    ) -> Result<EmailResponse, AppError> {
        tracing::debug!("Sending email to: {}", to);
        tracing::debug!("Email subject: {}", subject);
        tracing::debug!("Email body: {}", body);
        let email_options = CreateEmailBaseOptions::new(
            self.from_email.clone(),
            vec![to.to_string()],
            subject.to_string(),
        )
        .with_html(body);

        let response: CreateEmailResponse = self.client.emails.send(email_options).await?;

        tracing::debug!("Email send result: {:#?}", response);

        Ok(EmailResponse {
            id: response.id.to_string(),
            from: self.from_email.clone(),
            to: to.to_string(),
            subject: subject.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }
}

#[cfg(feature = "ssr")]
#[tokio::test]
#[ignore = "do not send email everytime"]
async fn resend_email_test() -> Result<(), AppError> {
    use std::str::FromStr;
    println!("Starting Resend email test");
    let resend = ResendClient::try_init().unwrap();
    let from_env = EmailAddress::from_str(&std::env::var("RESEND_FROM").unwrap())?;

    let result = resend
        .send_email(
            from_env,
            "Test Email",
            "<h1>Hello World</h1><p>This is a test email.</p>",
        )
        .await;

    println!("Email send result: {:#?}", result);

    Ok(())
}
