use crate::domain::guild::entities::error::GuildError;
use crate::domain::guild::entities::model::Guild;
use std::future::Future;

pub trait GuildService: Clone + Send + Sync + 'static {
    fn create_guild(
        &self,
        name: &str,
        owner_id: &str,
    ) -> impl Future<Output = Result<Guild, GuildError>> + Send;
    fn delete_guild(&self, name: &str) -> impl Future<Output = Result<(), GuildError>> + Send;
    fn find_by_name(&self, name: &str) -> impl Future<Output = Result<Guild, GuildError>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = Result<Guild, GuildError>> + Send;
    fn get_guilds(&self) -> impl Future<Output = Result<Vec<Guild>, GuildError>> + Send;
    fn update_guild(&self, name: &str) -> impl Future<Output = Result<(), GuildError>> + Send;
    fn delete_by_id(&self, id: &str) -> impl Future<Output = Result<(), GuildError>> + Send;
}

pub trait GuildRepository: Clone + Send + Sync + 'static {
    fn create_guild(
        &self,
        name: &str,
        owner_id: &str,
    ) -> impl Future<Output = Result<Guild, GuildError>> + Send;
    fn find_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Guild>, GuildError>> + Send;
    fn delete_by_id(&self, id: &str) -> impl Future<Output = Result<(), GuildError>> + Send;
}
