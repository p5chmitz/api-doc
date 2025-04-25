use super::handlers;
use crate::state::ApplicationState;
use axum::routing::{delete, get, patch, post};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/login",
            post(handlers::login_handler::login).with_state(state.clone()),
        )
        .route(
            "/patient",
            post(handlers::create_patient_handler::create)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::api::middleware::jwt::auth,
                )),
        )
        .route(
            "/patient/:patient_id",
            get(handlers::get_patient_handler::get_patient)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::api::middleware::jwt::auth,
                )),
        )
        .route(
            "/patient",
            get(handlers::list_patients_handler::list)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::api::middleware::jwt::auth,
                )),
        )
        .route(
            "/patient/:patient_id",
            patch(handlers::update_patient_handler::update)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::api::middleware::jwt::auth,
                )),
        )
        .route(
            "/patient/:patient_id",
            delete(handlers::delete_patient_handler::delete)
                .with_state(state.clone())
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::api::middleware::jwt::auth,
                )),
        )
}

// OAS doc
use utoipa::{
    openapi::
        security::{
            HttpAuthScheme, 
            HttpBuilder, 
            SecurityScheme
        },
        Modify, 
        OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::login_handler::login,
        handlers::create_patient_handler::create,
        handlers::get_patient_handler::get_patient,
        handlers::list_patients_handler::list,
        handlers::update_patient_handler::update,
        handlers::delete_patient_handler::delete,
    ),
    components(
        schemas(
            // Requests
            crate::api::request::login_request::LoginRequest,
            crate::api::request::create_patient_request::AddressCreate,
            crate::api::request::create_patient_request::BirthDateCreate,
            crate::api::request::create_patient_request::NameCreate,
            crate::api::request::create_patient_request::CreatePatientRequest,
            crate::api::request::update_patient_request::Address,
            crate::api::request::update_patient_request::BirthDate,
            crate::api::request::update_patient_request::Name,
            //crate::api::request::update_patient_request::UpdatePatientRequest,
            crate::api::request::update_patient_request::UpdatePatientRequestOas,

            // Responses
            crate::api::response::login_response::LoginResponse,
            crate::api::response::create_patient_response::AddressData,
            crate::api::response::create_patient_response::BirthdateData,
            crate::api::response::create_patient_response::NameData,
            crate::api::response::create_patient_response::Patient,
            crate::api::response::create_patient_response::CreatePatientResponse,
            crate::api::response::list_patients::AddressData,
            crate::api::response::list_patients::BirthdateData,
            crate::api::response::list_patients::NameData,
            crate::api::response::list_patients::Patient,
            crate::api::response::list_patients::ListPatientsResponse,
            crate::api::response::error::ErrorResponse,
        ),
    ),
    modifiers(&SecurityAddon),
    //tags(
    //    (name = "Auth", description = "Auth"),
    //    (name = "Patients", description = "Patients"),
    //),
    servers(
        (url = "/v1", description = "Localhost"),
    ),
)]
pub struct ApiDoc;

// Allows OpenAPI to use JWTs
struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
