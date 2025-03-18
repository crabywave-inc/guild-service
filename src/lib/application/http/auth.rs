use axum::http::StatusCode;
use axum::{body::Body, extract::Request, response::Response};
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use thiserror::Error;
use tower::{Layer, Service};
use tracing::{debug, error, info};

// Types publics pour l'identité utilisateur
#[derive(Clone, Debug)]
pub enum Identity {
    User(OidcUser),
    ServiceAccount(ServiceAccount),
}

#[derive(Clone, Debug)]
pub struct OidcUser {
    pub sub: String,
    pub email: Option<String>,
    pub preferred_username: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ServiceAccount {
    pub client_id: String,
}

// Erreurs d'authentification
#[derive(Debug, Clone, Error)]
pub enum AuthError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Failed to fetch decoding keys")]
    FetchingDecodingKeys,
    #[error("Failed to verify token")]
    VerifyingToken,
    #[error("Unable to get certificate")]
    UnableToGetCertificate,
    #[error("Missing authorization header")]
    MissingAuthHeader,
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

// Structures JWKS
#[derive(Debug, Serialize, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>
}

#[derive(Debug, Serialize, Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

// Structure des claims du token JWT
#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    pub sub: String,
    pub email: Option<String>,
    #[serde(default)]
    pub email_verified: bool,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub preferred_username: Option<String>,
    pub client_id: Option<String>,
    pub azp: Option<String>,
    pub exp: usize,
    pub iat: usize,
}

impl TokenClaims {
    fn is_service_account(&self) -> bool {
        self.client_id.is_some() || self.azp.is_some()
    }
    
    fn to_identity(&self) -> Identity {
        if self.is_service_account() {
            let client_id = self.client_id.clone()
                .or_else(|| self.azp.clone())
                .unwrap_or_else(|| "unknown".to_string());
                
            Identity::ServiceAccount(ServiceAccount { client_id })
        } else {
            Identity::User(OidcUser {
                sub: self.sub.clone(),
                email: self.email.clone(),
                preferred_username: self.preferred_username.clone(),
            })
        }
    }
}

// Cache pour les clés JWKS
struct JwksCache {
    keys: HashMap<String, DecodingKey>,
    last_updated: Instant,
}

// Configuration du middleware d'authentification
#[derive(Clone)]
pub struct AuthConfig {
    oidc_base_url: String,
    jwks_path: String,
    token_expiry_leeway: u64,
    disable_audience_validation: bool,
    expected_audiences: Vec<String>,
    jwks_refresh_interval: Duration,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            oidc_base_url: "http://localhost:8080/realms/crabywave".to_string(),
            jwks_path: "/protocol/openid-connect/certs".to_string(),
            token_expiry_leeway: 30, // 30 secondes de marge
            disable_audience_validation: true,
            expected_audiences: vec![],
            jwks_refresh_interval: Duration::from_secs(3600), // 1 heure
        }
    }
}

impl AuthConfig {
    pub fn new(oidc_base_url: String) -> Self {
        Self {
            oidc_base_url,
            ..Default::default()
        }
    }

    fn jwks_uri(&self) -> String {
        format!("{}{}", self.oidc_base_url, self.jwks_path)
    }
}


#[derive(Clone)]
pub struct AuthenticationLayer {
    config: AuthConfig,
    jwks_cache: Arc<RwLock<Option<JwksCache>>>,
}

impl AuthenticationLayer {
    pub fn new(oidc_base_url: String) -> Self {
        Self {
            config: AuthConfig::new(oidc_base_url),
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }
}

impl<S> Layer<S> for AuthenticationLayer {
    type Service = AuthenticationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticationMiddleware {
            inner,
            config: self.config.clone(),
            jwks_cache: self.jwks_cache.clone(),
        }
    }
}

// Middleware d'authentification
#[derive(Clone)]
pub struct AuthenticationMiddleware<S> {
    inner: S,
    config: AuthConfig,
    jwks_cache: Arc<RwLock<Option<JwksCache>>>,
}

impl<S> AuthenticationMiddleware<S> {
    pub fn new(inner: S, oidc_base_url: String) -> Self {
        Self {
            inner,
            config: AuthConfig::new(oidc_base_url),
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_config(inner: S, config: AuthConfig) -> Self {
        Self {
            inner,
            config,
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }
}

impl<S> Service<Request> for AuthenticationMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let config = self.config.clone();
        let jwks_cache = self.jwks_cache.clone();

        // Extraire le token
        let token = extract_token(&request);

        Box::pin(async move {
            let identity = match token {
                Some(token) => {
                    match authenticate_token(&token, &config, &jwks_cache).await {
                        Ok(identity) => {
                            debug!("Authentication successful for identity: {:?}", identity);
                            Some(identity)
                        },
                        Err(err) => {
                            error!("Authentication error: {}", err);
                            return Ok(build_error_response(err));
                        }
                    }
                },
                None => {
                    return Ok(build_error_response(AuthError::MissingAuthHeader));
                }
            };

            if let Some(identity) = identity {
                request.extensions_mut().insert(identity);
                inner.call(request).await
            } else {
                Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap())
            }
        })
    }
}

// Fonctions auxiliaires
fn extract_token(request: &Request) -> Option<String> {
    request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(String::from)
}

async fn authenticate_token(
    token: &str, 
    config: &AuthConfig,
    jwks_cache: &Arc<RwLock<Option<JwksCache>>>,
) -> Result<Identity, AuthError> {
    // Décodage de l'entête pour obtenir le kid
    let token_header = decode_header(token)
        .map_err(|e| AuthError::InvalidToken(e.to_string()))?;
    
    let kid = token_header.kid
        .ok_or_else(|| AuthError::InvalidToken("Missing 'kid' in token header".to_string()))?;

    // Récupération des clés
    let decoding_key = get_decoding_key(&kid, config, jwks_cache).await?;

    // Configuration de la validation
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    validation.leeway = config.token_expiry_leeway;
    
    if config.disable_audience_validation {
        validation.validate_aud = false;
    } else if !config.expected_audiences.is_empty() {
        validation.set_audience(&config.expected_audiences);
    }

    // Validation et décodage du token
    let token_data = decode::<TokenClaims>(token, &decoding_key, &validation)
        .map_err(|e| {
            error!("Token validation error: {:?}", e);
            AuthError::InvalidToken(e.to_string())
        })?;

    // Conversion des claims en identité
    Ok(token_data.claims.to_identity())
}

async fn get_decoding_key(
    kid: &str,
    config: &AuthConfig,
    jwks_cache: &Arc<RwLock<Option<JwksCache>>>,
) -> Result<DecodingKey, AuthError> {
    // Vérifier si nous avons déjà les clés en cache et si elles sont valides
    let should_refresh = {
        let cache_read = jwks_cache.read().unwrap();
        match &*cache_read {
            Some(cache) => {
                // Si la clé existe dans le cache et que le cache est récent, on l'utilise
                if cache.keys.contains_key(kid) && 
                   cache.last_updated.elapsed() < config.jwks_refresh_interval {
                    return Ok(cache.keys.get(kid).unwrap().clone());
                }
                true
            },
            None => true,
        }
    };

    // Si on doit rafraîchir le cache
    if should_refresh {
        debug!("Refreshing JWKS cache");
        let new_keys = fetch_jwks_keys(config).await?;
        
        // Mise à jour du cache
        let mut cache_write = jwks_cache.write().unwrap();
        *cache_write = Some(JwksCache {
            keys: new_keys.clone(),
            last_updated: Instant::now(),
        });

        // Vérifier si la clé existe dans les nouvelles clés
        if let Some(key) = new_keys.get(kid) {
            return Ok(key.clone());
        }
    }

    // Si on arrive ici, c'est que la clé n'a pas été trouvée
    error!("No matching key found for kid: {}", kid);
    Err(AuthError::InvalidToken(format!("No matching key found for kid: {}", kid)))
}

async fn fetch_jwks_keys(config: &AuthConfig) -> Result<HashMap<String, DecodingKey>, AuthError> {
    let client = Client::new();
    let jwks_uri = config.jwks_uri();
    
    debug!("Fetching JWKS from: {}", jwks_uri);
    
    let response = client.get(&jwks_uri)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to fetch JWKS: {}", e);
            AuthError::FetchingDecodingKeys
        })?;

    if !response.status().is_success() {
        error!("Failed to fetch JWKS, status: {}", response.status());
        return Err(AuthError::FetchingDecodingKeys);
    }

    let jwks: Jwks = response.json().await.map_err(|e| {
        error!("Failed to parse JWKS response: {}", e);
        AuthError::UnableToGetCertificate
    })?;

    let mut decoding_keys = HashMap::new();
    for key in jwks.keys {
        match DecodingKey::from_rsa_components(&key.n, &key.e) {
            Ok(decoding_key) => {
                decoding_keys.insert(key.kid.clone(), decoding_key);
                debug!("Added key with kid: {}", key.kid);
            },
            Err(e) => {
                error!("Failed to create decoding key for kid {}: {}", key.kid, e);
            }
        }
    }

    debug!("Fetched {} JWKS keys", decoding_keys.len());
    Ok(decoding_keys)
}

fn build_error_response(error: AuthError) -> Response {
    let (status, message) = match error {
        AuthError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),
        AuthError::MissingAuthHeader => (StatusCode::UNAUTHORIZED, "Missing authorization header"),
        AuthError::FetchingDecodingKeys => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch key material"),
        AuthError::VerifyingToken => (StatusCode::UNAUTHORIZED, "Failed to verify token"),
        AuthError::UnableToGetCertificate => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get certificate"),
        AuthError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
    };

    Response::builder()
        .status(status)
        .body(Body::from(message))
        .unwrap()
}