use crate::application::http::auth::Identity;
use crate::application::http::handlers::{ApiError, ApiSuccess};
use crate::domain::guild::entities::error::GuildError;
use crate::domain::guild::entities::model::Guild;
use crate::domain::guild::ports::GuildService;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CreateGuildResponseData(Guild);


#[derive(Serialize, TypedPath)]
#[typed_path("/v1/guilds")]
pub struct CreateGuildRoute;

#[derive(Deserialize)]
pub struct CreateGuildRequest {
    name: String,
}

pub async fn create_guild<G: GuildService>(
    _: CreateGuildRoute,
    Extension(guild_service): Extension<Arc<G>>,
    Extension(identity): Extension<Identity>,
    Json(payload): Json<CreateGuildRequest>,
) -> Result<ApiSuccess<CreateGuildResponseData>, ApiError> {
    let user = match identity {
        Identity::User(user) => user,
        _ => {
            return Err(ApiError::Forbidden(
                "You're not allowed to create a guild".to_string(),
            ))
        }
    };

    let user_id = Uuid::parse_str(&user.sub).map_err(|_| GuildError::NotFound)?;

    guild_service
        .create_guild(&payload.name, user_id)
        .await
        .map_err(ApiError::from)
        .map(|guild| ApiSuccess::new(StatusCode::CREATED, CreateGuildResponseData(guild)))
}
