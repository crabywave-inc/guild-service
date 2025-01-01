use axum::http::StatusCode;
use axum::{body::Body, extract::Request, response::Response};
use futures_util::future::BoxFuture;
use reqwest::Client;
use serde::Deserialize;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthenticationLayer {
    auth_service_url: String,
}

impl AuthenticationLayer {
    pub fn new(auth_service_url: String) -> Self {
        Self { auth_service_url }
    }
}

impl<S> Layer<S> for AuthenticationLayer {
    type Service = AuthenticationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticationMiddleware::new(inner, self.auth_service_url.clone())
    }
}

#[derive(Clone)]
pub struct AuthenticationMiddleware<S> {
    inner: S,
    auth_service_url: String,
}

impl<S> AuthenticationMiddleware<S> {
    pub fn new(inner: S, auth_service_url: String) -> Self {
        Self {
            inner,
            auth_service_url,
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
        let auth_service_url = self.auth_service_url.clone();
        let mut inner = self.inner.clone();

        let token = request
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(header["Bearer ".len()..].to_string())
                } else {
                    None
                }
            });

        if token.is_none() {
            return Box::pin(async {
                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Unauthorized"))
                    .unwrap();
                Ok(response)
            });
        }

        Box::pin(async move {
            match token {
                Some(token) => {
                    match verify_token_with_auth_service(&auth_service_url, &token).await {
                        Ok(data) => {
                            // Ajouter l'email dans les extensions
                            request.extensions_mut().insert(data);

                            // Passer la requête au middleware suivant
                            inner.call(request).await
                        }
                        Err(_) => {
                            // Si le token est invalide, retourner une réponse 401
                            let response = Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .body(Body::from("Invalid token"))
                                .unwrap();
                            Ok(response)
                        }
                    }
                }
                None => {
                    // Si le token est absent, retourner une réponse 401
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Missing token"))
                        .unwrap();
                    Ok(response)
                }
            }
        })
    }
}

#[derive(Deserialize)]
struct VerifyResponse {
    data: UserPayload,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserPayload {
    pub email: String,
    pub id: String,
}

async fn verify_token_with_auth_service(
    auth_service_url: &str,
    token: &str,
) -> Result<UserPayload, ()> {
    let client = Client::new();
    let response = client
        .post(format!("{}/auth/verify", auth_service_url))
        .json(&serde_json::json!({ "token": token }))
        .send()
        .await
        .map_err(|_| ())?;

    if response.status().is_success() {
        let verify_response: VerifyResponse = response.json().await.map_err(|_| ())?;
        Ok(verify_response.data)
    } else {
        Err(())
    }
}
