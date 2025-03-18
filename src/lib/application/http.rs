mod auth;
mod handlers;
mod policies;
mod responses;

use crate::application::http::handlers::create_guild::create_guild;
use crate::application::http::handlers::delete_guild::delete_guild;
use crate::application::http::handlers::get_guild::get_guild;
use auth::AuthenticationLayer;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum_extra::routing::RouterExt;
use handlers::{create_guild, get_user_guilds};
use handlers::get_user_guilds::get_user_guilds;

use crate::domain::guild::ports::GuildService;
use crate::env::Env;
use anyhow::Context;
use axum::routing::{delete, get, post};
use axum::{middleware, Extension};

use std::sync::Arc;
use tokio::net;
use tracing::{info, info_span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Clone)]
struct AppState<G>
where
    G: GuildService,
{
    guild_service: Arc<G>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<G>(
        config: HttpServerConfig,
        env: Arc<Env>,
        guild_service: Arc<G>,
    ) -> anyhow::Result<Self>
    where
        G: GuildService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState { guild_service };

        let auth_layer = AuthenticationLayer::new("t".to_string());

        let router = axum::Router::new()
            .merge(api_routes())
            .layer(trace_layer)
            .layer(auth_layer)
            .layer(Extension(Arc::clone(&state.guild_service)))
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!(
            "Server is running on http://{}",
            self.listener.local_addr()?
        );
        axum::serve(self.listener, self.router)
            .await
            .context("received error while running server")?;

        Ok(())
    }
}

fn api_routes<G>() -> axum::Router<AppState<G>>
where
    G: GuildService,
{
    axum::Router::new()
        .typed_get(create_guild::<G>)
        .typed_get(get_guild::<G>)
        .typed_get(get_user_guilds::<G>)
        .typed_delete(delete_guild::<G>)
}
