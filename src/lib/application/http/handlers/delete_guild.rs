use crate::application::http::policies::guild_policy::GuildPolicy;
use crate::{application::http::auth::UserPayload, domain::guild::ports::GuildService};
use axum::http::StatusCode;
use axum::{extract::Path, Extension};
use serde::Serialize;
use std::sync::Arc;

use crate::application::http::handlers::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct DeleteGuildResponseData {
    message: String,
}

pub async fn delete_guild<G: GuildService>(
    Extension(guild_service): Extension<Arc<G>>,
    Extension(user): Extension<UserPayload>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<DeleteGuildResponseData>, ApiError> {
  
    GuildPolicy::delete(&user.id, &id, Arc::clone(&guild_service))
      .await
      .map_err(|_| ApiError::Forbidden("You're not allowed to delete the guild".to_string()))?;

    guild_service
        .delete_by_id(&id)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::OK, DeleteGuildResponseData { message: "Guild deleted".to_string() }))
}
