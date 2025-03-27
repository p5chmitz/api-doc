use super::handlers;
use crate::state::ApplicationState;
use crate::api::handlers::jwt::auth;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route("/login", post(handlers::login_handler::login).with_state(state.clone()))
        .route(
            "/dogs",
            post(handlers::dog::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/patient",
            post(handlers::create_patient_handler::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(state, auth)),
        )
}
