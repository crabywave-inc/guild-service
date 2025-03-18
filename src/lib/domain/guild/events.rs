use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entities::model::Guild;

pub enum GuildEvent {
    Create,
    Delete,
}

pub enum RoleEvent {
    Create,
    Delete,
}

impl Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuildEvent::Create => write!(f, "guild-created"),
            GuildEvent::Delete => write!(f, "guild-deleted"),
        }
    }
}

impl Display for RoleEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleEvent::Create => write!(f, "role-created"),
            RoleEvent::Delete => write!(f, "role-deleted"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGuildMessageEvent {
    pub name: String,
    pub owner_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildDeletedMessageEvent {
    pub id: Uuid,
}

impl CreateGuildMessageEvent {
    pub fn from_guild(guild: &Guild) -> Self {
        Self {
            name: guild.name.clone(),
            owner_id: guild.owner_id.clone(),
            id: guild.id.clone(),
        }
    }
}

impl GuildDeletedMessageEvent {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
