#[cfg(feature = "ssr")]
use oauth2::basic::BasicClient;
#[cfg(feature = "ssr")]
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Github,
    Google,
    Discord,
}

impl OAuthProvider {
    pub fn as_str(&self) -> &str {
        match self {
            OAuthProvider::Github => "github",
            OAuthProvider::Google => "google",
            OAuthProvider::Discord => "discord",
        }
    }
}

#[cfg(feature = "ssr")]
pub struct OAuthConfig {
    pub provider: OAuthProvider,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
}

#[cfg(feature = "ssr")]
impl OAuthConfig {
    pub fn github() -> Result<Self, String> {
        Ok(Self {
            provider: OAuthProvider::Github,
            client_id: std::env::var("GITHUB_CLIENT_ID")
                .map_err(|_| "Missing GITHUB_CLIENT_ID environment variable")?,
            client_secret: std::env::var("GITHUB_CLIENT_SECRET")
                .map_err(|_| "Missing GITHUB_CLIENT_SECRET environment variable")?,
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            user_info_url: "https://api.github.com/user".to_string(),
        })
    }

    pub fn google() -> Result<Self, String> {
        Ok(Self {
            provider: OAuthProvider::Google,
            client_id: std::env::var("GOOGLE_CLIENT_ID")
                .map_err(|_| "Missing GOOGLE_CLIENT_ID environment variable")?,
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET")
                .map_err(|_| "Missing GOOGLE_CLIENT_SECRET environment variable")?,
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://www.googleapis.com/oauth2/v3/token".to_string(),
            user_info_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
        })
    }

    pub fn discord() -> Result<Self, String> {
        Ok(Self {
            provider: OAuthProvider::Discord,
            client_id: std::env::var("DISCORD_CLIENT_ID")
                .map_err(|_| "Missing DISCORD_CLIENT_ID environment variable")?,
            client_secret: std::env::var("DISCORD_CLIENT_SECRET")
                .map_err(|_| "Missing DISCORD_CLIENT_SECRET environment variable")?,
            auth_url: "https://discord.com/api/oauth2/authorize".to_string(),
            token_url: "https://discord.com/api/oauth2/token".to_string(),
            user_info_url: "https://discord.com/api/users/@me".to_string(),
        })
    }

    pub fn get_redirect_url() -> String {
        std::env::var("TINKR_AUTH_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
    }

    pub fn build_client(&self) -> Result<BasicClient, String> {
        let auth_url =
            AuthUrl::new(self.auth_url.clone()).map_err(|e| format!("Invalid auth URL: {}", e))?;
        let token_url = TokenUrl::new(self.token_url.clone())
            .map_err(|e| format!("Invalid token URL: {}", e))?;

        let redirect_url = format!(
            "{}/api/auth/callback/{}",
            Self::get_redirect_url(),
            self.provider.as_str()
        );

        let redirect =
            RedirectUrl::new(redirect_url).map_err(|e| format!("Invalid redirect URL: {}", e))?;

        Ok(BasicClient::new(
            ClientId::new(self.client_id.clone()),
            Some(ClientSecret::new(self.client_secret.clone())),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthUserInfo {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
}
