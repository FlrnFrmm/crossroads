use axum::{http::StatusCode, Json};

#[derive(Debug)]
pub(super) enum Error {
    HostAlreadyExists,
    DatabaseError(crate::database::error::Error),
    FailedToSendEvent,
    FailedToLoad(anyhow::Error),
}

impl From<Error> for (StatusCode, Json<serde_json::Value>) {
    fn from(error: Error) -> Self {
        let (status, message) = match error {
            Error::HostAlreadyExists => (
                StatusCode::CONFLICT,
                format!("Host already exists, use update instead"),
            ),
            Error::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::FailedToSendEvent => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send event".to_string(),
            ),
            Error::FailedToLoad(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to load: {}", e),
            ),
        };

        (status, Json(serde_json::json!({ "error": message })))
    }
}
