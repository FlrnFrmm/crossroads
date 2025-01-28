use axum::{http::StatusCode, Json};

#[derive(Debug)]
pub(super) enum ApiError {
    InvalidIp(String),
    DuplicateOrigin(String),
    NotFound,
}

impl From<ApiError> for (StatusCode, Json<serde_json::Value>) {
    fn from(error: ApiError) -> Self {
        let (status, message) = match error {
            ApiError::InvalidIp(ip) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid IPv4 address: {}", ip),
            ),
            ApiError::DuplicateOrigin(origin) => (
                StatusCode::CONFLICT,
                format!("Origin already exists: {}", origin),
            ),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Road not found".to_string()),
        };

        (status, Json(serde_json::json!({ "error": message })))
    }
}
