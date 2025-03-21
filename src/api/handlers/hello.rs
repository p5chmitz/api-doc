use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

/// Response generator for the /v1/hello endpoint
pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nHello perverts! Config: {}\n\n",
        state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("-".to_string())
    ))
}
