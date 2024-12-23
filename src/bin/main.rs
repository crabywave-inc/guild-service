use clap::Parser;

use guild::application::http::{HttpServer, HttpServerConfig};
use guild::application::ports::messaging_ports::{MessagingType, MessagingTypeImpl};
use guild::domain::guild::services::GuildServiceImpl;
use guild::env::Env;
use guild::infrastructure::db::firestore::Firestore;
use guild::infrastructure::guild::db::firestore_guild_repository::FirestoreGuildRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let _messaging_port =
        Arc::new(MessagingTypeImpl::new(&MessagingType::PubSub, Arc::clone(&env)).await?);

    let firestore = Arc::new(Firestore::new(Arc::clone(&env)).await?);
    let guild_repository = FirestoreGuildRepository::new(Arc::clone(&firestore));
    let guild_service = Arc::new(GuildServiceImpl::new(guild_repository));

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config, Arc::clone(&guild_service)).await?;

    http_server.run().await
}
