use std::sync::Arc;

use crate::application::ports::messaging_ports::MessagingPort;
use crate::domain::guild::entities::error::GuildError;
use crate::domain::guild::entities::model::Guild;
use crate::domain::guild::ports::{GuildRepository, GuildService};

#[derive(Debug, Clone)]
pub struct GuildServiceImpl<G, M>
where
    G: GuildRepository,
    M: MessagingPort
{
    pub guild_repository: G,
    pub messaging: Arc<M>
}

impl<G, M> GuildServiceImpl<G, M>
where
    G: GuildRepository,
    M: MessagingPort
{
    pub fn new(guild_repository: G, messaging: Arc<M>) -> Self {
        Self { guild_repository, messaging }
    }
}

impl<G, M> GuildService for GuildServiceImpl<G, M>
where
    G: GuildRepository,
    M: MessagingPort
{
    async fn create_guild(&self, name: &str, owner_id: &str) -> Result<Guild, GuildError> {
        let guild = self.guild_repository
            .create_guild(name, owner_id)
            .await?;

        // Create message in topic


        Ok(guild)
    }

    async fn delete_guild(&self, _name: &str) -> Result<(), GuildError> {
        todo!()
    }

    async fn find_by_name(&self, _name: &str) -> Result<Guild, GuildError> {
        todo!()
    }

    async fn find_by_id(&self, id: &str) -> Result<Guild, GuildError> {
        let guild = self.guild_repository.find_by_id(id).await?;

        match guild {
            Some(guild) => Ok(guild),
            None => Err(GuildError::NotFound),
        }
    }

    async fn get_guilds(&self) -> Result<Vec<Guild>, GuildError> {
        todo!()
    }

    async fn update_guild(&self, _name: &str) -> Result<(), GuildError> {
        todo!()
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), GuildError> {
        self.guild_repository.delete_by_id(id).await
    }
}
