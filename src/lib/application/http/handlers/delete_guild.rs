use crate::application::http::auth::Identity;
use crate::application::http::policies::guild_policy::GuildPolicy;
use crate::domain::guild::entities::error::GuildError;
use crate::domain::guild::ports::GuildService;
use axum::http::StatusCode;
use axum::{extract::Path, Extension};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::http::handlers::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct DeleteGuildResponseData {
    message: String,
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/v1/guilds/{id}")]
pub struct DeleteGuildRoute {
    id: Uuid,
}

pub async fn delete_guild<G: GuildService>(
    DeleteGuildRoute { id }: DeleteGuildRoute,
    Extension(guild_service): Extension<Arc<G>>,
    Extension(identity): Extension<Identity>,
) -> Result<ApiSuccess<DeleteGuildResponseData>, ApiError> {
    let user = match identity {
        Identity::User(user) => user,
        _ => {
            return Err(ApiError::Forbidden(
                "You're not allowed to delete the guild".to_string(),
            ))
        }
    };

    let user_id = Uuid::parse_str(&user.sub).map_err(|_| GuildError::NotFound)?;

    GuildPolicy::delete(user_id, id.clone(), Arc::clone(&guild_service))
        .await
        .map_err(|_| ApiError::Forbidden("You're not allowed to delete the guild".to_string()))?;

    guild_service
        .delete_by_id(id)
        .await
        .map_err(ApiError::from)
        .map(|_| {
            ApiSuccess::new(
                StatusCode::OK,
                DeleteGuildResponseData {
                    message: "Guild deleted".to_string(),
                },
            )
        })
}
