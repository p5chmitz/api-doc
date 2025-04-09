use crate::state::ApplicationState;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

//use utoipa_scalar::{Scalar, Servable};

mod handlers;
mod middleware;
mod request;
mod response;
//mod schemas;
mod v1;

pub const SWAGGER: &str = "/v1/swagger-ui";
pub const JSON: &str = "/v1/openapi.json";

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        // For Swagger UI
        .merge(SwaggerUi::new(SWAGGER).url(JSON, crate::api::v1::ApiDoc::openapi()))
        .nest("/v1", v1::configure(state))

    // For Scalar UI
    //Router::new()
    //    .merge(Scalar::with_url("/scalar", crate::api::v1::ApiDoc::openapi()))
}
