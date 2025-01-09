mod auth;
mod handlers;
mod policies;
mod responses;

use crate::application::http::auth::AuthenticationLayer;
use crate::application::http::handlers::create_guild::create_guild;
use crate::application::http::handlers::delete_guild::delete_guild;
use crate::application::http::handlers::get_guild::get_guild;
use handlers::get_user_guilds::get_user_guilds;

use crate::domain::guild::ports::GuildService;
use crate::env::Env;
use anyhow::Context;
use axum::routing::{delete, get, post};
use axum::Extension;

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

#[derive(Debug, Clone)]
struct AppState<G>
where
    G: GuildService,
{
    guild_service: Arc<G>,
    _auth_service_url: String,
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

        let state = AppState {
            guild_service,
            _auth_service_url: env.auth_service_url.clone(),
        };

        let auth_layer = AuthenticationLayer::new(env.auth_service_url.clone());

        let router = axum::Router::new()
            //.route("/", axum::handler::get(|| async { "Hello, World!" }))
            .nest("", api_routes())
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
        .route("/guilds", post(create_guild::<G>))
        .route("/guilds/:id", get(get_guild::<G>))
        .route("/guilds/:id", delete(delete_guild::<G>))
        .route("/users/@me/guilds", get(get_user_guilds::<G>))
}
