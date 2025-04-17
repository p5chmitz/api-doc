use crate::api::request::create_patient_request::CreatePatientRequest;
use crate::api::response::create_patient_response::{
    AddressData, BirthdateData, CreatePatientResponse, NameData, Patient,
};
//use chrono::NaiveDate;
use crate::api::response::error::AppError;
use crate::api::response::TokenClaims;
use crate::entities::patient;
use crate::state::ApplicationState;
use axum::{debug_handler, extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use std::sync::Arc;
use uuid::Uuid;
//use crate::api::response::error::ErrorResponse;
use crate::api::middleware::json::CustomJson;
use opentelemetry::{Key, Value};
use tracing::instrument;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Create a patient record
#[utoipa::path(
    post,
    path = "/patient",
    tag = "Patients",
    request_body = CreatePatientRequest,
    responses(
        (status = 200, description = "Success", body = CreatePatientResponse),
        (status = 400, description = "Generic error response format", body = ErrorResponse),
    ),
    security(
        ("api_jwt_token" = [])
    )
)]
#[debug_handler]
#[instrument(level = "info", name = "create_patient", skip_all)]
pub async fn create(
    Extension(claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    CustomJson(payload): CustomJson<CreatePatientRequest>,
) -> Result<Json<CreatePatientResponse>, AppError> {
    // Start a tracing span
    let span = Span::current();
    span.set_attribute(Key::from("http.method"), Value::from("POST"));
    let name = &claims.sub;
    span.set_attribute(Key::from("user"), Value::from(name.to_string()));

    // Validations
    let payload_ref = &payload;
    validate(payload_ref);

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
        sublocality: Set(payload.address.sublocality.unwrap_or("".to_string())),
        locality: Set(payload.address.locality.unwrap_or("".to_string())),
        administrative_area: Set(payload
            .address
            .administrative_area
            .unwrap_or("".to_string())),
        postal_code: Set(payload.address.postal_code.unwrap_or("".to_string())),
        country_region: Set(payload.address.country_region),
        ..Default::default()
    };
    let birthdate_active_model = patient::birthdate::ActiveModel {
        day: Set(payload.birth_date.day),
        month: Set(payload.birth_date.month),
        year: Set(payload.birth_date.year),
        ..Default::default()
    };

    // Stores Models
    let name_model: patient::name::Model = name_active_model.insert(db).await?;
    let address_model: patient::address::Model = address_active_model.insert(db).await?;
    let birthdate_model: patient::birthdate::Model = birthdate_active_model.insert(db).await?;
    let uuid = Uuid::new_v4(); // Creates the patient_record_id

    // Create and store the full patient record
    let patient_active_model = patient::ActiveModel {
        name_id: Set(name_model.id),
        address_id: Set(address_model.id),
        birthdate_id: Set(birthdate_model.id),
        patient_id: Set(uuid),
        ..Default::default()
    };

    let patient_model: patient::Model = patient_active_model.insert(db).await?;

    // Constructs response from generated models
    let response_data = Patient {
        created_at: patient_model.created_at.to_string(),
        patient_id: uuid.into(),
        name: NameData {
            first: name_model.first,
            middle: name_model.middle,
            surname: name_model.surname,
        },
        address: AddressData {
            address_lines: address_model.address_lines,
            sublocality: address_model.sublocality,
            locality: address_model.locality,
            administrative_area: address_model.administrative_area,
            postal_code: address_model.postal_code,
            country_region: address_model.country_region,
        },
        birthdate: BirthdateData {
            day: birthdate_model.day,
            month: birthdate_model.month,
            year: birthdate_model.year,
        },
    };

    span.set_attribute(
        Key::from("http.status_code"),
        Value::from(StatusCode::OK.as_u16() as i64),
    );
    Ok(Json(CreatePatientResponse {
        //status: 200,
        data: response_data,
    }))
}

fn validate(payload: &CreatePatientRequest) {
    // 1) Prints some test output to server
    if payload.name.first == "Peter".to_string() {
        println!("We got one!!!")
    };

    // 2) Calculates patient's age
    use chrono::{Datelike, NaiveDate, Utc};

    let today = Utc::now().date_naive();
    let _timestamp = Utc::now().format("%Y/%m/%d %H:%M:%S");
    let birth_date = NaiveDate::from_ymd_opt(
        payload.birth_date.year,
        payload.birth_date.month as u32,
        payload.birth_date.day as u32,
    )
    .expect("Invalid birth date");

    let mut age = today.year() - birth_date.year();

    // Adjust if birthday hasn't occurred yet this year
    if today < birth_date.with_year(today.year()).unwrap() {
        age -= 1;
    }

    println!("Patient is {} years old", age);
}
