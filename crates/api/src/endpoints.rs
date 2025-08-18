use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::database::errors::DatabaseError;
use crate::database::Database;
use crate::errors::ApiError;
use crate::road::Road;

pub(super) async fn all_roads(
    State(db): State<Arc<RwLock<Database>>>,
) -> Result<Json<Vec<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.read().await;
    let roads = db
        .all_roads()
        .await
        .map_err(|_| ApiError::DatabaseError(DatabaseError::UnableToReadRoads))?;
    Ok(Json(roads))
}

pub(super) async fn create_road(
    State(db): State<Arc<RwLock<Database>>>,
    Path(host): Path<String>,
    body: Bytes,
) -> Result<Json<Road>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let component = body.to_vec();
    let Some(road) = db
        .create_road(Road { host, component })
        .await
        .map_err(|_| ApiError::DatabaseError(DatabaseError::UnableToCreateRoad))?
    else {
        return Err(ApiError::HostAlreadyExists.into());
    };
    Ok(Json(road))
}

pub(super) async fn get_road(
    State(db): State<Arc<RwLock<Database>>>,
    Path(host): Path<String>,
) -> Result<Json<Option<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.read().await;
    let road = db
        .road_exists(&host)
        .await
        .map_err(|_| ApiError::DatabaseError(DatabaseError::UnableToReadRoads))?;
    Ok(Json(road))
}

pub(super) async fn update_road(
    State(db): State<Arc<RwLock<Database>>>,
    Path(host): Path<String>,
    body: Bytes,
) -> Result<Json<Option<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let component = body.to_vec();
    let updated_road = db
        .update_road(Road { host, component })
        .await
        .map_err(|_| ApiError::DatabaseError(DatabaseError::UnableToReadRoads))?;
    Ok(Json(updated_road))
}

pub(super) async fn delete_road(
    State(db): State<Arc<RwLock<Database>>>,
    Path(host): Path<String>,
) -> Result<Json<Option<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let deleted_road = db
        .delete_road(host)
        .await
        .map_err(|_| ApiError::DatabaseError(DatabaseError::UnableToReadRoads))?;
    Ok(Json(deleted_road))
}
