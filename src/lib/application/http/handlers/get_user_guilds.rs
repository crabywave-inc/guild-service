use std::sync::Arc;

use axum::http::StatusCode;
use axum::Extension;
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::http::auth::Identity;
use crate::domain::guild::entities::error::GuildError;
use crate::domain::guild::{entities::model::Guild, ports::GuildService};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize)]
pub struct GetUserGuildsResponseData(Vec<Guild>);

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/users/@me/guilds")]
pub struct GetUserGuilds;

pub async fn get_user_guilds<G: GuildService>(
    _: GetUserGuilds,
    Extension(guild_service): Extension<Arc<G>>,
    Extension(identity): Extension<Identity>,
) -> Result<ApiSuccess<GetUserGuildsResponseData>, ApiError> {
    let user = match identity {
        Identity::User(user) => user,
        _ => {
            return Err(ApiError::Forbidden(
                "You're not allowed to get the user guilds".to_string(),
            ))
        }
    };

    let user_id = Uuid::parse_str(&user.sub).map_err(|_| GuildError::NotFound)?;

    guild_service
        .find_by_user_id(user_id)
        .await
        .map_err(ApiError::from)
        .map(|guilds| ApiSuccess::new(StatusCode::OK, GetUserGuildsResponseData(guilds)))
}
