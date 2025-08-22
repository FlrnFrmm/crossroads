mod loader;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

use crate::database::error::Error as DbErr;
use crate::database::Database;
use crate::error::Error as ApiErr;
use crate::road::event::Event as RoadEvent;
use crate::road::Road;
use loader::Loader;

pub(super) async fn all_roads(
    State((db, _)): State<(Arc<RwLock<Database>>, Sender<RoadEvent>)>,
) -> Result<Json<Vec<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.read().await;
    let roads = db
        .all_roads()
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    Ok(Json(roads))
}

pub(super) async fn create_road(
    State((db, sender)): State<(Arc<RwLock<Database>>, Sender<RoadEvent>)>,
    Path(host): Path<String>,
    Json(loader): Json<Loader>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let component = loader.load().map_err(|e| ApiErr::FailedToLoad(e))?;
    let Some(road) = db
        .create_road(Road { host, component })
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToCreateRoad))?
    else {
        return Err(ApiErr::HostAlreadyExists.into());
    };
    sender
        .send(RoadEvent::Create(road))
        .await
        .map_err(|_| ApiErr::FailedToSendEvent)?;
    Ok(StatusCode::CREATED)
}

pub(super) async fn get_road(
    State((db, _)): State<(Arc<RwLock<Database>>, Sender<RoadEvent>)>,
    Path(host): Path<String>,
) -> Result<Json<Option<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.read().await;
    let road = db
        .road_exists(&host)
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    Ok(Json(road))
}

pub(super) async fn update_road(
    State((db, _)): State<(Arc<RwLock<Database>>, Sender<RoadEvent>)>,
    Path(host): Path<String>,
    Json(loader): Json<Loader>,
) -> Result<Json<Option<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let component = loader.load().map_err(|e| ApiErr::FailedToLoad(e))?;
    let updated_road = db
        .update_road(Road { host, component })
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    Ok(Json(updated_road))
}

pub(super) async fn delete_road(
    State((db, _)): State<(Arc<RwLock<Database>>, Sender<RoadEvent>)>,
    Path(host): Path<String>,
) -> Result<Json<Option<Road>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let deleted_road = db
        .delete_road(host)
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToDeleteRoad))?;
    Ok(Json(deleted_road))
}
