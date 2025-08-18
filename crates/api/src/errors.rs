use axum::{http::StatusCode, Json};

#[derive(Debug)]
pub(super) enum ApiError {
    HostAlreadyExists,
    DatabaseError(crate::database::errors::DatabaseError),
    FailedToSendEvent,
}

impl From<ApiError> for (StatusCode, Json<serde_json::Value>) {
    fn from(error: ApiError) -> Self {
        let (status, message) = match error {
            ApiError::HostAlreadyExists => (
                StatusCode::CONFLICT,
                format!("Host already exists, use update instead"),
            ),
            ApiError::DatabaseError(db_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, db_err.to_string())
            }
            ApiError::FailedToSendEvent => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send event".to_string(),
            ),
        };

        (status, Json(serde_json::json!({ "error": message })))
    }
}
