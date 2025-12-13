mod loader;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::database::Database;
use crate::database::error::Error as DbErr;
use crate::error::Error as ApiErr;
use loader::Loader;
use runtime::Runtime;
use runtime::proxy::ProxyMetadata;

pub(super) async fn current_proxy(
    State((db, _)): State<(Arc<RwLock<Database>>, Runtime)>,
) -> Result<Json<Option<ProxyMetadata>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.read().await;
    let proxy = db
        .get_current_proxy()
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    Ok(Json(proxy))
}

pub(super) async fn set_current_proxy(
    State((db, runtime)): State<(Arc<RwLock<Database>>, Runtime)>,
    Path(tag): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let maybe_proxy_metadata = db
        .set_current_proxy(&tag)
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    let Some(proxy) = maybe_proxy_metadata else {
        return Ok(StatusCode::NOT_FOUND);
    };

    runtime
        .set_proxy(&proxy.component)
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    Ok(StatusCode::OK)
}

pub(super) async fn all_proxies(
    State((db, _)): State<(Arc<RwLock<Database>>, Runtime)>,
) -> Result<Json<Vec<ProxyMetadata>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.read().await;
    let proxys = db
        .all_proxies()
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    Ok(Json(proxys))
}

pub(super) async fn create_proxy(
    State((db, _)): State<(Arc<RwLock<Database>>, Runtime)>,
    Path(tag): Path<String>,
    Json(loader): Json<Loader>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let component = loader.load().map_err(|e| ApiErr::FailedToLoad(e))?;
    let Some(_tag) = db
        .create_proxy(tag, component.clone())
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToCreateRoad))?
    else {
        return Err(ApiErr::TagAlreadyExists.into());
    };
    Ok(StatusCode::CREATED)
}

pub(super) async fn get_proxy(
    State((db, _)): State<(Arc<RwLock<Database>>, Runtime)>,
    Path(tag): Path<String>,
) -> Result<Json<Option<ProxyMetadata>>, (StatusCode, Json<serde_json::Value>)> {
    let db = db.read().await;
    let proxy_metadata = db
        .proxy_exists(&tag)
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    Ok(Json(proxy_metadata))
}

pub(super) async fn update_proxy(
    State((db, runtime)): State<(Arc<RwLock<Database>>, Runtime)>,
    Path(tag): Path<String>,
    Json(loader): Json<Loader>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    let component = loader.load().map_err(|e| ApiErr::FailedToLoad(e))?;
    let Some(proxy_metadata) = db
        .update_proxy(tag, component.clone())
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToUpdateRoad))?
    else {
        return Ok(StatusCode::NOT_FOUND);
    };
    let current_proxy_metadata = db
        .get_current_proxy()
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?
        .ok_or(ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
    if current_proxy_metadata.tag == proxy_metadata.tag {
        let current_proxy = db
            .get_proxy(&current_proxy_metadata.tag)
            .await
            .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?
            .ok_or(ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
        runtime
            .set_proxy(&current_proxy.component)
            .map_err(|_| ApiErr::FailedToSendMessage)?;
    }
    Ok(StatusCode::OK)
}

pub(super) async fn delete_proxy(
    State((db, runtime)): State<(Arc<RwLock<Database>>, Runtime)>,
    Path(tag): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let db = db.write().await;
    if let Some(proxy_metadata) = db
        .delete_proxy(tag)
        .await
        .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToDeleteRoad))?
    {
        let current_proxy_metadata = db
            .get_current_proxy()
            .await
            .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?
            .ok_or(ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
        if current_proxy_metadata.tag == proxy_metadata.tag {
            let current_proxy = db
                .get_proxy(&current_proxy_metadata.tag)
                .await
                .map_err(|_| ApiErr::DatabaseError(DbErr::UnableToReadRoads))?
                .ok_or(ApiErr::DatabaseError(DbErr::UnableToReadRoads))?;
            runtime
                .set_proxy(&current_proxy.component)
                .map_err(|_| ApiErr::FailedToSendMessage)?;
        }
    }
    Ok(StatusCode::OK)
}
