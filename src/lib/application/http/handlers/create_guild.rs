use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::guild::entities::model::Guild;
use crate::domain::guild::ports::GuildService;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CreateGuildResponseData(Guild);

#[derive(Deserialize)]
pub struct CreateGuildRequest {
    name: String,
}

pub async fn create_guild<G: GuildService>(
    Extension(guild_service): Extension<Arc<G>>,
    Json(payload): Json<CreateGuildRequest>,
) -> Result<ApiSuccess<CreateGuildResponseData>, ApiError> {
    guild_service
        .create_guild(&payload.name)
        .await
        .map_err(ApiError::from)
        .map(|guild| ApiSuccess::new(StatusCode::CREATED, CreateGuildResponseData(guild)))
}
