use crate::application::http::policies::guild_policy::GuildPolicy;
use crate::{application::http::auth::UserPayload, domain::guild::ports::GuildService};
use axum::http::StatusCode;
use axum::{extract::Path, Extension};
use std::sync::Arc;

use super::{ApiError, ApiSuccess};

pub async fn delete_guild<G: GuildService>(
    Extension(guild_service): Extension<Arc<G>>,
    Extension(user): Extension<UserPayload>,
    Path(id): Path<String>,
) -> Result<ApiSuccess<String>, ApiError> {
    GuildPolicy::delete(&user.id, &id, Arc::clone(&guild_service))
      .await
      .map_err(|_| ApiError::Forbidden("You're not allowed to delete the guild".to_string()))?;

    guild_service
        .delete_by_id(&id)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::NO_CONTENT, "Deleted".to_string()))
}
