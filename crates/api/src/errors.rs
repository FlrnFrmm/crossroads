use axum::{http::StatusCode, Json};

#[derive(Debug)]
pub(super) enum ApiError {
    InvalidIp(String),
    HostAlreadyExists,
    NotFound,
    DatabaseError(crate::database::errors::DatabaseError),
}

impl From<ApiError> for (StatusCode, Json<serde_json::Value>) {
    fn from(error: ApiError) -> Self {
        let (status, message) = match error {
            ApiError::InvalidIp(ip) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid IPv4 address: {}", ip),
            ),
            ApiError::HostAlreadyExists => (
                StatusCode::CONFLICT,
                format!("Host already exists, use update instead"),
            ),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Road not found".to_string()),
            ApiError::DatabaseError(db_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, db_err.to_string())
            }
        };

        (status, Json(serde_json::json!({ "error": message })))
    }
}
