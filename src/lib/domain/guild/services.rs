use crate::domain::guild::entities::error::GuildError;
use crate::domain::guild::entities::model::Guild;
use crate::domain::guild::ports::{GuildRepository, GuildService};

#[derive(Debug, Clone)]
pub struct GuildServiceImpl<G>
where
    G: GuildRepository,
{
    pub guild_repository: G,
}

impl<G> GuildServiceImpl<G>
where
    G: GuildRepository,
{
    pub fn new(guild_repository: G) -> Self {
        Self { guild_repository }
    }
}

impl<G> GuildService for GuildServiceImpl<G>
where
    G: GuildRepository,
{
    async fn create_guild(&self, name: &str) -> Result<Guild, GuildError> {
        self.guild_repository
            .create_guild(name)
            .await
            .map(|guild| Ok(guild))?
    }

    async fn delete_guild(&self, _name: &str) -> Result<(), GuildError> {
        todo!()
    }

    async fn find_by_name(&self, _name: &str) -> Result<Guild, GuildError> {
        todo!()
    }

    async fn find_by_id(&self, _id: &str) -> Result<Guild, GuildError> {
        todo!()
    }

    async fn get_guilds(&self) -> Result<Vec<Guild>, GuildError> {
        todo!()
    }

    async fn update_guild(&self, _name: &str) -> Result<(), GuildError> {
        todo!()
    }
}
