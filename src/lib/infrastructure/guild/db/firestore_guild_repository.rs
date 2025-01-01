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
    async fn create_guild(&self, name: &str, owner_id: &str) -> Result<Guild, GuildError> {
        let id = uuid::Uuid::new_v4().to_string();
        let guild = Guild {
            id,
            name: name.to_string(),
            owner_id: owner_id.to_string(),
            banner: "".to_string(),
            description: "".to_string(),
            icon: "".to_string(),
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

    async fn find_by_id(&self, id: &str) -> Result<Option<Guild>, GuildError> {
        let guild: Option<Guild> = self
            .firestore
            .db
            .fluent()
            .select()
            .by_id_in("guilds")
            .obj()
            .one(id)
            .await
            .map_err(|_| GuildError::NotFound)?;

        Ok(guild)
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), GuildError> {
        self.firestore
            .db
            .fluent()
            .delete()
            .from("guilds")
            .document_id(id)
            .execute()
            .await
            .map_err(|e| GuildError::DeleteError(e.to_string()))?;

        Ok(())
    }
}
