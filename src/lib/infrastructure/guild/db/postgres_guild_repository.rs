use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::guild::{
        entities::{error::GuildError, model::Guild},
        ports::GuildRepository,
    },
    infrastructure::db::postgres::Postgres,
};

#[derive(Debug, Clone)]
pub struct PostgresGuildRepository {
    pub postgres: Arc<Postgres>,
}

impl PostgresGuildRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

impl GuildRepository for PostgresGuildRepository {
    async fn create_guild(&self, name: &str, owner_id: Uuid) -> Result<Guild, GuildError> {
        let guild = Guild::from_service(name, owner_id);

        sqlx::query!(
            r#"
        INSERT INTO guilds (
          id, 
          name, 
          owner_id, 
          icon, 
          description, 
          banner
        ) VALUES ($1, $2, $3, $4, $5, $6)
        "#,
            guild.id,
            guild.name,
            guild.owner_id,
            guild.icon,
            guild.description,
            guild.banner
        )
        .execute(&*self.postgres.get_pool())
        .await
        .map_err(|_| GuildError::InternalServerError)?;

        Ok(guild)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), GuildError> {
        sqlx::query!(
            r#"
      DELETE FROM guilds WHERE id = $1
      "#,
            id
        )
        .execute(&*self.postgres.get_pool())
        .await
        .map_err(|_| GuildError::InternalServerError)?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Guild>, GuildError> {
        let guild = sqlx::query_as!(
            Guild,
            r#"
      SELECT * FROM guilds WHERE id = $1
      "#,
            id
        )
        .fetch_optional(&*self.postgres.get_pool())
        .await
        .map_err(|_| GuildError::InternalServerError)?;

        Ok(guild)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Guild>, GuildError> {
        let guilds = sqlx::query_as!(
            Guild,
            r#"
      SELECT * FROM guilds WHERE owner_id = $1
      "#,
            user_id
        )
        .fetch_all(&*self.postgres.get_pool())
        .await
        .map_err(|_| GuildError::InternalServerError)?;

        Ok(guilds)
    }
}
