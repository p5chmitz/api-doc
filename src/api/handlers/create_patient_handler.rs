use crate::api::request::create_patient_request::CreatePatientRequest;
use crate::api::response::create_patient_response::CreatePatientResponse;
//use chrono::NaiveDate;
use crate::entities::patient;
use crate::api::response::error::AppError;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::{debug_handler, Extension, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use std::sync::Arc;

#[debug_handler]
pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<CreatePatientRequest>,
) -> Result<Json<CreatePatientResponse>, AppError> {
    // Open a DB connection
    let db_conn = state.db_conn.load();
    let db = db_conn.as_ref();

    // Convert request payload to `ActiveModel`
    let name_active_model = patient::name::ActiveModel {
        first: Set(payload.name.first),
        middle: Set(payload.name.middle.unwrap_or("".to_string())),
        surname: Set(payload.name.surname),
        ..Default::default()
    };
    let address_active_model = patient::address::ActiveModel {
        address_lines: Set(payload.address.address_lines),
        sublocality: Set(payload.address.sublocality
            .unwrap_or("".to_string())),
        locality: Set(payload.address.locality
            .unwrap_or("".to_string())),
        administrative_area: Set(payload.address.administrative_area
            .unwrap_or("".to_string())),
        postal_code: Set(payload.address.postal_code
            .unwrap_or("".to_string())),
        country_region: Set(payload.address.country_region),
        ..Default::default()
    };
    let birthdate_active_model = patient::birthdate::ActiveModel {
        day: Set(payload.birth_date.day),
        month: Set(payload.birth_date.month),
        year: Set(payload.birth_date.year),
        ..Default::default()
    };

    // Insert and retrieve Models
    let name_model: patient::name::Model = 
        name_active_model.insert(db).await?;
    
    let address_model: patient::address::Model = 
        address_active_model.insert(db).await?;
    
    let birthdate_model: patient::birthdate::Model = 
        birthdate_active_model.insert(db).await?;
    
    // Insert the patient record and associate with the previous models
    let patient_active_model = patient::ActiveModel {
        name_id: Set(name_model.id),
        address_id: Set(address_model.id),
        birthdate_id: Set(birthdate_model.id),
        ..Default::default()
    };
    let patient_model: patient::Model = patient_active_model.insert(db).await?;

    Ok(Json(CreatePatientResponse {
        status: "success".to_string(),
        data: patient_model, // This is the full patient::Model
    }))
}
