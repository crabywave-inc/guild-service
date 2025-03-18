use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OidcConfig {
    pub issuer: String,
    pub jwks_uri: String,
    pub client_id: String,
    pub insecure: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub enum AuthConfig {
    #[serde(rename = "oidc")]
    OIDC(OidcConfig),
}
