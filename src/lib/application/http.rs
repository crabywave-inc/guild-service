mod handlers;
mod responses;

use crate::application::http::handlers::create_guild::create_guild;
use crate::domain::guild::ports::GuildService;
use anyhow::Context;
use axum::routing::post;
use axum::Extension;
use std::sync::Arc;
use tokio::net;
use tracing::{info, info_span};
use crate::application::http::handlers::get_guild::get_guild;

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
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<G>(config: HttpServerConfig, guild_service: Arc<G>) -> anyhow::Result<Self>
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

        let router = axum::Router::new()
            //.route("/", axum::handler::get(|| async { "Hello, World!" }))
            .nest("", api_routes())
            .layer(trace_layer)
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
    axum::Router::new().route("/guilds", post(create_guild::<G>))
        .route("/guilds/:guild_id", post(get_guild::<G>))
}
