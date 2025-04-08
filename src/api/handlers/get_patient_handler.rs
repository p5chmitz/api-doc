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
    Extension, Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/patient/{patient_id}",
    params(
        // utoipa doesn't support uuid directly, so the path param
        // has to be a String instead
        ("patient_id" = String, Path, description = "Patient ID as UUID v4", example = "3973ebb8-11e5-4725-93b7-3b752caad60f
")
    ),
    tag = "Patients",
    responses(
        (status = 200, description = "Success", body = CreatePatientResponse),
        (status = 400, description = "Generic error response format", body = ErrorResponse),
    ),
    security(
        ("api_jwt_token" = [])
    )
)]
#[debug_handler]
pub async fn get_patient(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Path(patient_id): Path<Uuid>,
) -> Result<Json<CreatePatientResponse>, AppError> {
    // Create a DB connection binding to share it
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
            // If the search returns a hit, fetch its data,
            // assemble the JSON, and return it
            if let Some(model) = conn {
                // Fetch related name
                let name = name::Entity::find_by_id(model.name_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| {
                        AppError(StatusCode::NOT_FOUND, anyhow!("Name record not found"))
                    })?;

                // Fetch related address
                let address = address::Entity::find_by_id(model.address_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| {
                        AppError(StatusCode::NOT_FOUND, anyhow!("Address record not found"))
                    })?;

                // Fetch related birthdate
                let birthdate = birthdate::Entity::find_by_id(model.birthdate_id)
                    .one(db)
                    .await?
                    .ok_or_else(|| {
                        AppError(StatusCode::NOT_FOUND, anyhow!("Birthdate record not found"))
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
                return Ok(Json(CreatePatientResponse {
                    data: response_data,
                }));
            // If the search is Ok, but there is no hit,
            // return a 404 NOT_FOUND error
            } else {
                return Err(AppError(
                    StatusCode::NOT_FOUND,
                    anyhow!("Patient {patient_id} not found"),
                ));
            }
        }
        // If the search is not Ok, issue a generic DB connection error
        // and obfuscate the specifics
        Err(_) => {
            return Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow!("Uh oh..."),
            ))
        }
    }
}
