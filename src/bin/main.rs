use clap::Parser;

use std::sync::Arc;
use guild::application::http::{HttpServer, HttpServerConfig};
use guild::application::ports::messaging_ports::{MessagingType, MessagingTypeImpl};
use guild::env::Env;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let _messaging_port = Arc::new(MessagingTypeImpl::new(&MessagingType::PubSub, Arc::clone(&env)).await?);

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await
}
