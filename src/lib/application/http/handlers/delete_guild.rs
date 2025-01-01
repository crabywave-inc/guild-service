use crate::{application::http::auth::UserPayload, domain::guild::ports::GuildService};
use axum::http::StatusCode;
use axum::{extract::Path, Extension};
use std::sync::Arc;

use super::{ApiError, ApiSuccess};

pub async fn delete_guild<G: GuildService>(
    Extension(guild_service): Extension<Arc<G>>,
    Extension(user): Extension<UserPayload>,
    Path(guild_id): Path<String>,
) -> Result<ApiSuccess<String>, ApiError> {
    guild_service
        .delete_by_id(&guild_id)
        .await
        .map_err(ApiError::from)
        .map(|_| ApiSuccess::new(StatusCode::NO_CONTENT, "Deleted".to_string()))
}
