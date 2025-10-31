use crate::server::{
    auth::extractor::AuthenticatedUser,
    config::AppState,
    networks::types::Network,
    shared::types::api::{ApiError, ApiResponse, ApiResult},
};
use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_network))
        .route("/", get(get_all_networks))
        .route("/{id}", put(update_network))
        .route("/{id}", delete(delete_network))
}

async fn create_network(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(request): Json<Network>,
) -> ApiResult<Json<ApiResponse<Network>>> {
    tracing::info!("Received network creation request: {:?}", request.base.name);

    if let Err(validation_errors) = request.base.validate() {
        tracing::error!("Network validation failed: {:?}", validation_errors);
        return Err(ApiError::bad_request(&format!(
            "Network validation failed: {}",
            validation_errors
        )));
    }

    let service = &state.services.network_service;
    let created_network = service.create_network(request).await?;

    Ok(Json(ApiResponse::success(created_network)))
}

async fn get_all_networks(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
) -> ApiResult<Json<ApiResponse<Vec<Network>>>> {
    let service = &state.services.network_service;

    let networks = service.get_all_networks(&user.0).await?;

    Ok(Json(ApiResponse::success(networks)))
}

async fn update_network(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(request): Json<Network>,
) -> ApiResult<Json<ApiResponse<Network>>> {
    let service = &state.services.network_service;

    let mut network = service
        .get_network(&id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Network '{}' not found", &id)))?;

    network.base = request.base;

    let updated_network = service.update_network(network).await?;

    Ok(Json(ApiResponse::success(updated_network)))
}

async fn delete_network(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<()>>> {
    let service = &state.services.network_service;

    // Check if network exists
    if service.get_network(&id).await?.is_none() {
        return Err(ApiError::not_found(format!("Network '{}' not found", &id)));
    }

    service.delete_network(&id).await?;

    Ok(Json(ApiResponse::success(())))
}
