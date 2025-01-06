use clap::Parser;

use guild::application::http::{HttpServer, HttpServerConfig};
use guild::application::ports::messaging_ports::{MessagingType, MessagingTypeImpl};
use guild::domain::guild::services::GuildServiceImpl;
use guild::env::{AppEnv, Env};
use guild::infrastructure::db::firestore::Firestore;
use guild::infrastructure::guild::db::firestore_guild_repository::FirestoreGuildRepository;
use std::sync::Arc;

fn init_logger(env: Arc<Env>) {
    match env.env {
        AppEnv::Development => {
            tracing_subscriber::fmt::init();
        }
        AppEnv::Production => {
            tracing_subscriber::fmt()
            .json()
            .with_max_level(tracing::Level::INFO)
            .init();
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let env = Arc::new(Env::parse());

    init_logger(Arc::clone(&env));

    let _messaging_port =
        Arc::new(MessagingTypeImpl::new(&MessagingType::PubSub, Arc::clone(&env)).await?);

    let firestore = Arc::new(Firestore::new(Arc::clone(&env)).await?);
    let guild_repository = FirestoreGuildRepository::new(Arc::clone(&firestore));
    let guild_service = Arc::new(GuildServiceImpl::new(guild_repository));

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server =
        HttpServer::new(server_config, Arc::clone(&env), Arc::clone(&guild_service)).await?;

    http_server.run().await
}
