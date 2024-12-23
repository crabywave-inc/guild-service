use crate::domain::guild::entities::error::GuildError;
use crate::domain::guild::entities::model::Guild;
use crate::domain::guild::ports::GuildRepository;
use crate::infrastructure::db::firestore::Firestore;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FirestoreGuildRepository {
    pub firestore: Arc<Firestore>,
}

impl FirestoreGuildRepository {
    pub fn new(firestore: Arc<Firestore>) -> Self {
        Self { firestore }
    }
}

impl GuildRepository for FirestoreGuildRepository {
    async fn create_guild(&self, name: &str) -> Result<Guild, GuildError> {
        let id = uuid::Uuid::new_v4().to_string();
        let guild = Guild {
            id,
            name: name.to_string(),
            owner_id: "test".to_string(),
        };

        self.firestore
            .db
            .fluent()
            .insert()
            .into("guilds")
            .document_id(&guild.id)
            .object(&guild)
            .execute::<()>()
            .await
            .map_err(|e| GuildError::CreateError(e.to_string()))?;

        Ok(guild)
    }
}
