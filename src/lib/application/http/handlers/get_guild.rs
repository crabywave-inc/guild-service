use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::guild::entities::model::Guild;
use crate::domain::guild::ports::GuildService;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Extension;
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize)]
pub struct GetGuildResponseData(Guild);

#[derive(Deserialize, TypedPath)]
#[typed_path("/v1/guilds/{id}")]
pub struct GetGuildRoute {
    id: Uuid,
}

pub async fn get_guild<G: GuildService>(
    GetGuildRoute { id }: GetGuildRoute,
    Extension(guild_service): Extension<Arc<G>>,
) -> Result<ApiSuccess<GetGuildResponseData>, ApiError> {
    guild_service
        .find_by_id(id)
        .await
        .map_err(ApiError::from)
        .map(|guild| ApiSuccess::new(StatusCode::OK, GetGuildResponseData(guild)))
}
