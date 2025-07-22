use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    api::{
        errors::AppError,
        model::{CreatePostRequest, UpdatePostRequest},
        response::posts::{ListPostResponse, SinglePostResponse},
    },
    service::post_service::PostService,
    state::ApplicationState,
};

pub async fn create(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.create_post(payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn update(
    Path(id): Path<i64>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.update_post(id, payload).await?;

    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn list(
    State(state): State<Arc<ApplicationState>>,
) -> Result<Json<ListPostResponse>, AppError> {
    let posts = state.post_service.get_all_posts().await?;
    let response = ListPostResponse { data: posts };

    Ok(Json(response))
}

pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<SinglePostResponse>, AppError> {
    let post = state.post_service.get_post_by_id(id).await?;
    let response = SinglePostResponse { data: post };

    Ok(Json(response))
}

pub async fn delete(
    State(state): State<Arc<ApplicationState>>,
    Path(id): Path<i64>,
) -> Result<Json<()>, AppError> {
    state.post_service.delete_post(id).await?;

    Ok(Json(()))
}
