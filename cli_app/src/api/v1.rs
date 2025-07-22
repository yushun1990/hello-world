use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::state::ApplicationState;

use super::handlers::{self};

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/hello", get(handlers::hello).with_state(state.clone()))
        .route("/posts", post(handlers::create).with_state(state.clone()))
        .route("/posts", get(handlers::list).with_state(state.clone()))
        .route("/posts/:id", get(handlers::get).with_state(state.clone()))
        .route(
            "/posts/:id",
            put(handlers::update).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            delete(handlers::delete).with_state(state.clone()),
        )
}
