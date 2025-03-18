use std::sync::Arc;

use scylla::{Session, SessionBuilder};
use tracing::info;

#[derive(Debug)]
pub struct Scylla {
    pub session: Session,
}

impl Scylla {
    pub async fn new(nodes: Vec<String>, keyspace: &str) -> Result<Arc<Self>, anyhow::Error> {
        let initial_session = SessionBuilder::new().known_nodes(&nodes).build().await?;

        let create_keyspace_query = format!(
          "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class': 'SimpleStrategy', 'replication_factor': 1}}",
          keyspace
      );

        info!("Creating keyspace if it doesn't exist: {}", keyspace);

        initial_session
            .query_unpaged(create_keyspace_query, &[])
            .await?;

        info!(
            "Connecting to ScyllaDB at {:?} with keyspace {}",
            nodes, keyspace
        );

        let session = SessionBuilder::new()
            .known_nodes(&nodes)
            .use_keyspace(keyspace, true)
            .build()
            .await?;

        info!("Successfully connected to ScyllaDB");

        Ok(Arc::new(Self { session }))
    }
}
