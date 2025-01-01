use std::fmt::Display;

pub enum GuildEvent {
    Create,
}

pub enum RoleEvent {
    Create,
    Delete,
}

impl Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuildEvent::Create => write!(f, "guild-created"),
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
