use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::guild::entities::model::Guild;
use crate::domain::guild::ports::GuildService;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Extension;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize)]
pub struct GetGuildResponseData(Guild);

pub async fn get_guild<G: GuildService>(
    Extension(guild_service): Extension<Arc<G>>,
    Path(guild_id): Path<String>,
) -> Result<ApiSuccess<GetGuildResponseData>, ApiError> {
    guild_service
        .find_by_id(&guild_id)
        .await
        .map_err(ApiError::from)
        .map(|guild| ApiSuccess::new(StatusCode::OK, GetGuildResponseData(guild)))
}
