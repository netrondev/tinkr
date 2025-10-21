mod address;

mod resend;

pub use address::EmailAddress;

#[cfg(feature = "ssr")]
use crate::AppError;

pub use resend::{EmailResponse, EmailResultDB};

#[cfg(feature = "ssr")]
pub async fn send_email(
    to: EmailAddress,
    subject: &str,
    body: &str,
) -> Result<EmailResponse, AppError> {
    let client = resend::ResendClient::try_init()?;
    let result = client.send_email(to, subject, body).await?;
    Ok(result)
}
