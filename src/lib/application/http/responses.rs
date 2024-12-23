use crate::application::http::handlers::ApiError;
use crate::domain::guild::entities::error::GuildError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseData {
    pub message: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorResponse {
    pub errors: Vec<ApiErrorDetail>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiErrorDetail {
    pub message: String,
    pub rule: String,
    pub field: String,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
pub struct ApiResponseError {
    pub code: String,
    pub status: u16,
    pub message: String,
}

impl From<GuildError> for ApiError {
    fn from(value: GuildError) -> Self {
        match value {
            GuildError::InternalServerError => {
                ApiError::InternalServerError("Internal server error".to_string())
            }
            GuildError::NotFound => ApiError::NotFound("Guild not found".to_string()),
            GuildError::CreateError(e) => {
                ApiError::UnProcessableEntity(e)
            }
            GuildError::Unauthorized => ApiError::Unauthorized("Unauthorized".to_string()),
        }
    }
}
