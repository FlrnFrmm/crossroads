use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::errors::ApiError;

pub type Roads = Arc<RwLock<HashMap<String, Vec<Ipv4Addr>>>>;

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Road {
    host: String,
    destination: Vec<String>,
}

pub(super) async fn create_road(
    State(state): State<Roads>,
    Json(road): Json<Road>,
) -> Result<Json<Road>, (StatusCode, Json<serde_json::Value>)> {
    let mut destinations = Vec::with_capacity(road.destination.len());
    for ip_string in road.destination.iter() {
        let ip = ip_string
            .parse::<Ipv4Addr>()
            .map_err(|_| ApiError::InvalidIp(ip_string.clone()))?;
        destinations.push(ip);
    }
    let mut roads = state.write().await;
    if roads.contains_key(&road.host) {
        return Err(ApiError::DuplicateOrigin(road.host).into());
    }
    roads.insert(road.host.clone(), destinations);
    Ok(Json(road))
}

pub(super) async fn all_roads(State(roads): State<Roads>) -> Json<Vec<Road>> {
    let all_roads = roads
        .read()
        .await
        .iter()
        .map(|(host, destinations)| Road {
            host: host.clone(),
            destination: destinations.iter().map(|ip| ip.to_string()).collect(),
        })
        .collect();
    Json(all_roads)
}

pub(super) async fn get_road(
    State(roads): State<Roads>,
    Path(host): Path<String>,
) -> Result<Json<Road>, (StatusCode, Json<serde_json::Value>)> {
    let roads = roads.read().await;
    let road = roads
        .get(&host)
        .ok_or(ApiError::NotFound)
        .map(|destinations| Road {
            host: host.clone(),
            destination: destinations.iter().map(|ip| ip.to_string()).collect(),
        })?;
    Ok(Json(road))
}

pub(super) async fn update_road(
    State(roads): State<Roads>,
    Path(host): Path<String>,
    Json(new_destinations): Json<Vec<String>>,
) -> Result<Json<Road>, (StatusCode, Json<serde_json::Value>)> {
    let mut new_destinations_parsed: Vec<Ipv4Addr> = Vec::with_capacity(new_destinations.len());
    for ip_string in new_destinations.iter() {
        let ip = ip_string
            .parse::<Ipv4Addr>()
            .map_err(|_| ApiError::InvalidIp(ip_string.clone()))?;
        new_destinations_parsed.push(ip);
    }
    let mut roads = roads.write().await;
    let destinations = roads.get_mut(&host).ok_or(ApiError::NotFound)?;
    *destinations = new_destinations_parsed;
    let road = Road {
        host,
        destination: new_destinations,
    };
    Ok(Json(road))
}

pub(super) async fn delete_road(
    State(roads): State<Roads>,
    Path(host): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let mut roads = roads.write().await;
    roads.remove(&host);
    Ok(StatusCode::NO_CONTENT)
}
