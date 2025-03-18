use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Guild {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
}

impl Guild {
    pub fn from_service(name: &str, owner_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            owner_id,
            icon: None,
            description: None,
            banner: None,
        }
    }
}
