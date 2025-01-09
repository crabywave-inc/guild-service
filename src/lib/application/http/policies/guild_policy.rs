use std::sync::Arc;

use tracing::warn;

use crate::domain::guild::{entities::error::GuildError, ports::GuildService};

pub struct GuildPolicy;

impl GuildPolicy {
    pub async fn delete<G: GuildService>(
        user_id: &str,
        guild_id: &str,
        guild_service: Arc<G>,
    ) -> Result<(), anyhow::Error> {
        let guild = guild_service
            .find_by_id(guild_id)
            .await
            .map_err(|_| GuildError::Forbidden)?;

        if guild.owner_id != user_id {
            warn!(
                "User {} tried to delete guild {} without permission",
                user_id, guild_id
            );
            return Err(GuildError::Forbidden.into());
        }
        Ok(())
    }
}
