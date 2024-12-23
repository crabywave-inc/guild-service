use std::fmt::Display;

pub enum GuildEvent {
    Create,
}

impl Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuildEvent::Create => write!(f, "guild-created"),
        }
    }
}
