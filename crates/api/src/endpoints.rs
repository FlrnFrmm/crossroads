use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use bytes::Bytes;

use crate::database::Database;
use crate::errors::{ApiError, DatabaseError};
use crate::road::Road;

// pub(super) async fn create_road(
//     State(db): State<Database>,
//     Path(host): Path<String>,
//     body: Bytes,
// ) -> Result<Json<Road>, (StatusCode, Json<serde_json::Value>)> {
//     let new_road = Road {
//         host: host.clone(),
//         component: body.to_vec(),
//     };
//     let Some(road) = db
//         .add_road(new_road)
//         .await
//         .map_err(|_| ApiError::DatabaseError(DatabaseError::UnableToCreateRoad))?
//     else {
//         return Err(ApiError::DuplicateOrigin(host).into());
//     };
//     Ok(Json(road))
// }

pub(super) async fn all_roads(
    State(db): State<Database>,
) -> Result<Json<Vec<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let roads = db
        .all_roads()
        .await
        .map_err(|_| ApiError::DatabaseError(DatabaseError::UnableToCreateRoad))?;
    Ok(Json(roads))
}

// pub(super) async fn get_road(
//     State(db): State<Database>,
//     Path(host): Path<String>,
// ) -> Result<Json<Road>, (StatusCode, Json<serde_json::Value>)> {
//     todo!()
// }

// pub(super) async fn update_road(
//     State(db): State<Database>,
//     Path(host): Path<String>,
//     body: Bytes,
// ) -> Result<Json<Road>, (StatusCode, Json<serde_json::Value>)> {
//     todo!()
// }

// pub(super) async fn delete_road(
//     State(db): State<Database>,
//     Path(host): Path<String>,
// ) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
//     todo!()
// }
