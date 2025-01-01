use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub icon: String,
    pub description: String,
    pub banner: String,
}
