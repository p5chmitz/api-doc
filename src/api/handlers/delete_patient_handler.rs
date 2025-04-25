use crate::api::response::create_patient_response::{
    AddressData, BirthdateData, CreatePatientResponse, NameData, Patient,
};
use crate::api::response::error::AppError;
use crate::api::response::TokenClaims;
use crate::entities::patient::{self, address, birthdate, name};
use crate::state::ApplicationState;
use anyhow::anyhow;
use axum::{
    debug_handler, 
    extract::{Path, State}, 
    http::StatusCode, 
    Extension, 
    Json
};
use sea_orm::{
    ActiveModelTrait, 
    ActiveValue::Set, 
    ColumnTrait, 
    EntityTrait, 
    QueryFilter
};
use std::sync::Arc;
use uuid::Uuid;
use opentelemetry::{Key, Value};
use tracing::instrument;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Delete a patient record
///
/// Delete a patient record by ID. The operation returns the deleted patient record as
/// confirmation.
#[utoipa::path(
    delete,
    path = "/patient/{patient_id}",
    params(
        // utoipa doesn't support uuid directly, so the path param
        // has to be a String instead
        ("patient_id" = String, Path, description = "Patient ID as UUID v4", example = "3973ebb8-11e5-4725-93b7-3b752caad60f
")
    ),
    tag = "Patient Records",
    responses(
        (status = 200, description = "Success", body = CreatePatientResponse),
        (status = 400, description = "Generic error response format", body = ErrorResponse),
    ),
    security(
        ("api_jwt_token" = [])
    )
)]
#[debug_handler]
#[instrument(level = "info", name = "delete_patient", skip_all)]
pub async fn delete(
    Extension(claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Path(patient_id): Path<Uuid>,
) -> Result<Json<CreatePatientResponse>, AppError> {
    // Start a tracing span
    let span = Span::current();
    span.set_attribute(Key::from("http.method"), Value::from("DELETE"));
    let name = &claims.sub;
    span.set_attribute(Key::from("user"), Value::from(name.to_string()));

    // Open a DB connection
    let db_conn = state.db_conn.load();
    let db = db_conn.as_ref();

    // Query the patient by UUID
    match patient::Entity::find()
        .filter(patient::Column::PatientId.eq(patient_id))
        .one(state.db_conn.load().as_ref())
        .await
    {
        // await returns Result<Option<Model>, DbErr>
        // so you have to safely unwrap all its thorny layers
        Ok(conn) => {
            // If the search returns a hit, set the
            // "deleted" flag to true, fetch the patient 
            // record data, assemble the JSON, 
            // and return it
            if let Some(model) = conn {

                // Set the "deleted" flag
                let mut active: patient::ActiveModel = model.clone().into();
                active.active_flag = Set(false);
                let _ = active.update(db).await?;

                // Fetch related name
                let name = name::Entity::find_by_id(model.name_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| {
                        let code = StatusCode::NOT_FOUND;
                        span.set_attribute(
                            Key::from("http.status_code"),
                            Value::from(code.as_u16() as i64),
                        );
                        span.set_attribute(
                            Key::from("request.payload"),
                            Value::from(format!("{:?}", &patient_id)),
                        );
                        AppError(code, anyhow!("Birthdate record not found"))
                    })?;

                // Fetch related address
                let address = address::Entity::find_by_id(model.address_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| {
                        let code = StatusCode::NOT_FOUND;
                        span.set_attribute(
                            Key::from("http.status_code"),
                            Value::from(code.as_u16() as i64),
                        );
                        span.set_attribute(
                            Key::from("request.payload"),
                            Value::from(format!("{:?}", &patient_id)),
                        );
                        AppError(code, anyhow!("Birthdate record not found"))
                    })?;

                // Fetch related birthdate
                let birthdate = birthdate::Entity::find_by_id(model.birthdate_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| {
                        let code = StatusCode::NOT_FOUND;
                        span.set_attribute(
                            Key::from("http.status_code"),
                            Value::from(code.as_u16() as i64),
                        );
                        span.set_attribute(
                            Key::from("request.payload"),
                            Value::from(format!("{:?}", &patient_id)),
                        );
                        AppError(code, anyhow!("Birthdate record not found"))
                    })?;

                // Construct the response
                let response_data = Patient {
                    patient_id: model.patient_id.into(),
                    created_at: model.created_at.to_rfc3339(),
                    name: NameData {
                        first: name.first,
                        middle: name.middle,
                        surname: name.surname,
                    },
                    address: AddressData {
                        address_lines: address.address_lines,
                        sublocality: address.sublocality,
                        locality: address.locality,
                        administrative_area: address.administrative_area,
                        postal_code: address.postal_code,
                        country_region: address.country_region,
                    },
                    birthdate: BirthdateData {
                        day: birthdate.day,
                        month: birthdate.month,
                        year: birthdate.year,
                    },
                };
                // Happy path
                span.set_attribute(
                    Key::from("http.status_code"),
                    Value::from(StatusCode::OK.as_u16() as i64),
                );
                return Ok(Json(CreatePatientResponse {
                    data: response_data,
                }));
            // If the search is Ok, but there is no hit,
            // return a 404 NOT_FOUND error
            } else {
                let code = StatusCode::NOT_FOUND;
                span.set_attribute(
                    Key::from("http.status_code"),
                    Value::from(code.as_u16() as i64),
                );
                span.set_attribute(
                    Key::from("request.payload"),
                    Value::from(format!("{:?}", &patient_id)),
                );
                return Err(AppError(code, anyhow!("Patient {patient_id} not found")));
            }
        }
        // If the search is not Ok, issue a generic DB connection error
        // and obfuscate the specifics
        Err(_) => {
            let code = StatusCode::INTERNAL_SERVER_ERROR;
            span.set_attribute(
                Key::from("http.status_code"),
                Value::from(code.as_u16() as i64),
            );
            span.set_attribute(
                Key::from("request.payload"),
                Value::from(format!("{:?}", &patient_id)),
            );
            return Err(AppError(code, anyhow!("Uh oh...")));
        }
    }
}
