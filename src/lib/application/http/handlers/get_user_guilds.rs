use std::sync::Arc;

use axum::http::StatusCode;
use axum::Extension;
use serde::Serialize;

use crate::application::http::auth::UserPayload;
use crate::domain::guild::{entities::model::Guild, ports::GuildService};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize)]
pub struct GetUserGuildsResponseData(Vec<Guild>);

pub async fn get_user_guilds<G: GuildService>(
    Extension(guild_service): Extension<Arc<G>>,
    Extension(user): Extension<UserPayload>
) -> Result<ApiSuccess<GetUserGuildsResponseData>, ApiError> {
    guild_service
        .find_by_user_id(&user.id)
        .await
        .map_err(ApiError::from)
        .map(|guilds| ApiSuccess::new(StatusCode::OK, GetUserGuildsResponseData(guilds)))
}
