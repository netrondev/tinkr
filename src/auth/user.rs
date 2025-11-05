use crate::{EmailAddress, session::get_user};

use leptos::prelude::*;

use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::{
    session::{AdapterSession, CreateSessionData},
    token::{CreateVerificationToken, VerificationToken},
    wallet::Wallet,
};

#[cfg(feature = "ssr")]
use crate::AppError;

#[cfg(feature = "ssr")]
use chrono::Utc;

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(not(feature = "ssr"))]
use crate::{Datetime, RecordId};

use crate::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize, Partial, PartialEq)]
#[partial(
    "CreateUserData",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(id, is_admin, superadmin)
)]
#[partial(
    "UpdateUserData",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(is_admin, superadmin)
)]
#[partial(
    "DeliveryDetails",
    derive(Debug, Serialize, Deserialize, Clone, PartialEq),
    omit(id, name, email_verified, is_admin, superadmin, theme, image)
)]
pub struct AdapterUser {
    pub id: RecordId,
    pub name: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: Option<Datetime>,
    pub image: Option<String>,
    pub email: EmailAddress,
    pub is_admin: Option<bool>,
    pub superadmin: Option<bool>,
    #[serde(default)]
    pub theme: Theme,

    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub postcode: Option<String>,
    pub phone: Option<String>,
    pub telephone: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
}

impl Default for DeliveryDetails {
    fn default() -> Self {
        Self {
            first_name: None,
            last_name: None,
            email: EmailAddress::create_blank(),
            address1: None,
            address2: None,
            address3: None,
            postcode: None,
            phone: None,
            telephone: None,
        }
    }
}

#[cfg(feature = "ssr")]
use crate::db_init;

#[cfg(feature = "ssr")]
impl AdapterUser {
    pub async fn create_user(user_data: CreateUserData) -> Result<Self, AppError> {
        use tracing::debug;

        let client = db_init().await?;

        debug!("Creating user with data: {:#?}", user_data);
        // if !user_data.email.is_empty() {
        //     let user = AdapterUser::get_user_by_email(user_data.email.clone()).await;
        //     if let Ok(_) = user {
        //         return Err(AppError::AuthError("Email already in use".into()));
        //     }
        // }
        debug!("Saving user to db");
        let create_result: Option<Self> = client.create("user").content(user_data).await?;
        let created: Self =
            create_result.ok_or_else(|| AppError::AuthError("Could not create user".into()))?;
        Ok(created)
    }

    pub async fn new_guest() -> Result<Self, AppError> {
        let user = Self::create_user(CreateUserData {
            email: EmailAddress::create_blank(),
            email_verified: None,
            name: format!("guest_{}", uuid::Uuid::new_v4()),
            image: None,
            theme: Theme::System,
            address1: None,
            address2: None,
            address3: None,
            postcode: None,
            phone: None,
            telephone: None,
            first_name: None,
            last_name: None,
        })
        .await?;
        Ok(user)
    }

    pub async fn create_test_user() -> Result<Self, AppError> {
        let user = Self::create_user(CreateUserData {
            email: EmailAddress::create_test_email(),
            email_verified: None,
            name: "Test User".to_string(),
            image: None,
            theme: Theme::System,
            address1: None,
            address2: None,
            address3: None,
            postcode: None,
            phone: None,
            telephone: None,
            first_name: None,
            last_name: None,
        })
        .await?;
        Ok(user)
    }

    pub fn is_super_admin(&self) -> Result<bool, AppError> {
        if let Some(superadmin) = self.superadmin {
            Ok(superadmin)
        } else {
            Err(AppError::AuthError("User is not a super admin".into()))
        }
    }

    pub async fn get_user(id: RecordId) -> Result<Self, AppError> {
        let client = db_init().await?;

        if id.table() != "user" {
            return Err(AppError::AuthError("Invalid user ID".into()));
        }

        let result: Option<Self> = client.select(id).await?;

        match result {
            Some(user) => Ok(user),
            None => Err(AppError::AuthError("User not found".into())),
        }
    }

    pub async fn get_user_by_email(email: EmailAddress) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query("SELECT * FROM ONLY user WHERE email = $email LIMIT 1;")
            .bind(("email", email))
            .await?;

        let user: Option<Self> = result.take(0)?;

        match user {
            Some(user) => Ok(user),
            None => Err(AppError::AuthError("User not found".into())),
        }
    }

    pub async fn get_by_email(email: String) -> Result<Self, AppError> {
        use std::str::FromStr;
        let email_address = EmailAddress::from_str(&email)
            .map_err(|_| AppError::AuthError("Invalid email address".into()))?;
        Self::get_user_by_email(email_address).await
    }

    pub async fn get_user_by_account(
        provider_account_id: RecordId,
    ) -> Result<Option<AdapterUser>, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query(
                "SELECT * FROM ONLY account WHERE providerAccountId = $providerAccountId LIMIT 1;",
            )
            .bind(("providerAccountId", provider_account_id))
            .await?;

        let user: Option<Self> = result.take(0)?;

        Ok(user)
    }

    pub async fn get_user_from_session(session_token: String) -> Result<Self, AppError> {
        use crate::db_seperate_connection;

        let client = db_seperate_connection().await?;

        let mut result = client
            .query("(SELECT user_id from ONLY session where session_token = $session_token LIMIT 1 FETCH user_id).user_id;")
            .bind(("session_token", session_token))
            .await?;

        let user: Option<Self> = result.take(0)?;

        if let Some(user) = user {
            Ok(user)
        } else {
            Err(AppError::AuthError(
                "User not found for session_token".into(),
            ))
        }
    }

    pub async fn set_verified_email(&self) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut user_update = client
            .query("UPDATE $userid SET email_verified = time::now() RETURN AFTER;")
            .bind(("userid", self.id.clone()))
            .await?;

        let user: Option<Self> = user_update.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn update_user(data: UpdateUserData) -> Result<AdapterUser, AppError> {
        let db = db_init().await?;

        let mut query = db
            .query("UPDATE $userid SET name = $name, email = $email, image = $image RETURN AFTER;")
            .bind(("userid", data.id.clone()))
            .bind(("name", data.name))
            .bind(("email", data.email.to_string()))
            .bind(("image", data.image))
            .await?;

        let user: Option<Self> = query.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn update_user_theme(&self, theme: Theme) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut query = client
            .query("UPDATE $userid SET theme = $theme RETURN AFTER;")
            .bind(("userid", self.id.clone()))
            .bind(("theme", theme))
            .await?;

        let user: Option<Self> = query.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn update_user_image(&self, image: String) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut query = client
            .query("UPDATE $userid SET image = $image RETURN AFTER;")
            .bind(("userid", self.id.clone()))
            .bind(("image", image))
            .await?;

        let user: Option<Self> = query.take(0)?;
        let user = user.ok_or_else(|| AppError::AuthError("User not found".into()))?;
        Ok(user)
    }

    pub async fn delete_user(&self) -> Result<(), AppError> {
        let client = db_init().await?;
        let _: Option<AdapterUser> = client.delete(&self.id).await?;
        // delete all related data?
        Ok(())
    }

    /// Creates a new verification token for the user.
    pub async fn new_verification_token(&self) -> Result<VerificationToken, AppError> {
        let token = VerificationToken::create_verification_token(CreateVerificationToken {
            email: self.email.clone(),
            user_id: self.id.clone(),
        })
        .await?;

        Ok(token)
    }

    pub async fn new_session(&self) -> Result<AdapterSession, AppError> {
        let session_data = CreateSessionData {
            user_id: self.id.clone(),
            session_token: uuid::Uuid::new_v4().to_string(),
            expires: Datetime::from(Utc::now() + chrono::Duration::days(365)),
        };

        AdapterSession::create_session(session_data).await
    }

    pub async fn get_all_users() -> Result<Vec<Self>, AppError> {
        let client = db_init().await?;
        let users: Vec<Self> = client.select("user").await?;
        Ok(users)
    }

    pub async fn wallets(&self) -> Result<Vec<Wallet>, AppError> {
        Wallet::get_by_user(self.id.clone()).await
    }

    pub async fn check_email_availability(&self, email: String) -> Result<bool, AppError> {
        use std::str::FromStr;
        let email_address = EmailAddress::from_str(&email)
            .map_err(|_| AppError::AuthError("Invalid email address".into()))?;

        let user = Self::get_user_by_email(email_address.clone()).await;
        if let Ok(user) = user {
            // If we found a user with this email and it's not the current user, it's not available
            if user.id != self.id {
                return Ok(false);
            }
        }

        // If the email is the same as the current user's, it's available
        if self.email == email_address {
            return Ok(true);
        }

        let client = db_init().await?;

        let mut result = client
            .query("SELECT count() as count FROM user WHERE email = $email;")
            .bind(("email", email_address.to_string()))
            .await?;

        #[derive(serde::Deserialize)]
        struct CountResult {
            count: i64,
        }

        let count: Option<CountResult> = result.take(0)?;
        let is_available = count.map_or(true, |c| c.count == 0);

        Ok(is_available)
    }

    pub async fn check_username_availability(username: String) -> Result<bool, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query("SELECT count() as count FROM user WHERE name = $name;")
            .bind(("name", username.clone()))
            .await?;

        #[derive(serde::Deserialize)]
        struct CountResult {
            count: i64,
        }

        let count: Option<CountResult> = result.take(0)?;
        let is_available = count.map_or(true, |c| c.count == 0);

        Ok(is_available)
    }

    pub async fn get_user_by_oauth_id(
        oauth_id: &str,
        provider: &crate::auth::oauth::OAuthProvider,
    ) -> Result<Self, AppError> {
        let client = db_init().await?;

        let mut result = client
            .query("SELECT VALUE ->links->user FROM oauth_account WHERE provider_account_id = $oauth_id AND provider = $provider LIMIT 1;")
            .bind(("oauth_id", oauth_id.to_string()))
            .bind(("provider", provider.as_str().to_string()))
            .await?;

        let user_ids: Option<Vec<RecordId>> = result.take(0)?;

        if let Some(ids) = user_ids {
            if let Some(user_id) = ids.first() {
                return Self::get_user(user_id.clone()).await;
            }
        }

        Err(AppError::AuthError("User not found".into()))
    }

    pub async fn link_oauth_account(
        user_id: &RecordId,
        oauth_id: &str,
        provider: &crate::auth::oauth::OAuthProvider,
    ) -> Result<(), AppError> {
        let client = db_init().await?;

        client
            .query("CREATE oauth_account CONTENT { provider_account_id: $oauth_id, provider: $provider, user: $user_id } RETURN NONE;")
            .bind(("oauth_id", oauth_id.to_string()))
            .bind(("provider", provider.as_str().to_string()))
            .bind(("user_id", user_id.clone()))
            .await?;

        Ok(())
    }
}

#[server]
pub async fn check_username_availability(username: String) -> Result<bool, ServerFnError> {
    Ok(AdapterUser::check_username_availability(username.clone()).await?)
}

#[server]
pub async fn check_email_availability(email: String) -> Result<bool, ServerFnError> {
    let current_user = get_user().await?;
    let is_available = current_user.check_email_availability(email.clone()).await?;
    Ok(is_available)
}

#[server]
pub async fn update_user_profile(
    name: String,
    email: String,
) -> Result<AdapterUser, ServerFnError> {
    use crate::EmailAddress;
    use std::str::FromStr;

    let user = get_user().await?;

    // Check if name is available (if changed)
    if user.name != name {
        let name_available = check_username_availability(name.clone()).await?;
        if !name_available {
            return Err(ServerFnError::ServerError(
                "Username is already taken".to_string(),
            ));
        }
    }

    // Check if email is available (if changed)
    let email_changed = user.email.0 != email;
    if email_changed {
        let email_available = check_email_availability(email.clone()).await?;
        if !email_available {
            return Err(ServerFnError::ServerError(
                "Email is already in use".to_string(),
            ));
        }
    }

    // Parse email
    let email_address = match EmailAddress::from_str(&email) {
        Ok(addr) => addr,
        Err(_) => {
            return Err(ServerFnError::ServerError(
                "Invalid email format".to_string(),
            ));
        }
    };

    // If email changed, reset verification status
    let email_verified = if email_changed {
        None
    } else {
        user.email_verified
    };

    // Create update data
    let update_data = UpdateUserData {
        id: user.id.clone(),
        name,
        email_verified,
        image: user.image,
        email: email_address,
        theme: user.theme,
        address1: None,
        address2: None,
        address3: None,
        postcode: None,
        phone: None,
        telephone: None,
        first_name: None,
        last_name: None,
    };

    // Update user
    let updated_user = AdapterUser::update_user(update_data).await?;

    // If email changed, send verification email
    if email_changed {
        // Send verification email (non-blocking, ignore errors)
        let _ = send_verification_email().await;
    }

    Ok(updated_user)
}

#[server]
pub async fn send_verification_email() -> Result<(), ServerFnError> {
    use crate::email::send_email;

    let user = get_user().await?;

    // Check if email is already verified
    if user.email_verified.is_some() {
        return Err(ServerFnError::ServerError(
            "Email is already verified".to_string(),
        ));
    }

    // Create verification token
    let token = user.new_verification_token().await?;

    // Construct verification URL
    let base_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let verification_url = format!(
        "{}/api/auth/callback/email-verify?token={}",
        base_url, token.token
    );

    // Send verification email
    let email_body = format!(
        r#"<html>
        <body>
            <h2>Verify Your Email</h2>
            <p>Hello {},</p>
            <p>Please click the link below to verify your email address:</p>
            <p><a href="{}">Verify Email</a></p>
            <p>Or copy and paste this URL into your browser:</p>
            <p>{}</p>
            <p>This link will expire in 1 hour.</p>
        </body>
        </html>"#,
        user.name, verification_url, verification_url
    );

    send_email(user.email.clone(), "Verify Your Email", &email_body).await?;

    Ok(())
}
