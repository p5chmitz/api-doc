use crate::api::request::update_patient_request::UpdatePatientRequest;
use crate::api::response::{
    create_patient_response::{
        AddressData, 
        BirthdateData, 
        CreatePatientResponse, 
        NameData, 
        Patient
    },
    error::AppError
};
use crate::api::response::TokenClaims;
use crate::entities::patient;
use crate::state::ApplicationState;
use crate::api::middleware::json::CustomJson;

use anyhow::anyhow;
use axum::{
    debug_handler, 
    extract::{Path, State}, 
    http::StatusCode, 
    Extension, 
    Json
};
use opentelemetry::{Key, Value};
use sea_orm::{
    ActiveModelTrait, 
    ActiveValue::Set,
    ColumnTrait, 
    EntityTrait, 
    QueryFilter
};
use std::sync::Arc;
use tracing::instrument;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

//macro_rules! set_if_some {
//    ($model:expr, $field:ident, $value:expr) => {
//        if let Some(ref v) = $value {
//            $model.$field = Set(v);
//        }
//    };
//}

/// Update a patient record
///
/// Update all fields for a given patient record aside from `name.first`, `name.surname`, and `birtdate`
#[utoipa::path(
    patch,
    path = "/patient/{patient_id}",
    tag = "Patient Records",
    params(
        // utoipa doesn't support uuid directly, so the path param
        // has to be a String instead
        ("patient_id" = String, Path, description = "Patient ID as UUID v4", example = "3973ebb8-11e5-4725-93b7-3b752caad60f
")
    ),
    request_body = UpdatePatientRequestOas,
    responses(
        (status = 200, description = "Success", body = CreatePatientResponse),
        (status = 400, description = "Generic error response format", body = ErrorResponse),
    ),
    security(
        ("api_jwt_token" = [])
    )
)]
#[debug_handler]
#[instrument(level = "info", name = "update_patient", skip_all)]
pub async fn update(
    Extension(claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Path(patient_id): Path<Uuid>,
    CustomJson(payload): CustomJson<UpdatePatientRequest>
) -> Result<Json<CreatePatientResponse>, AppError> {
    // Start a tracing span
    let span = Span::current();
    span.set_attribute(Key::from("http.method"), Value::from("PATCH"));
    let name = &claims.sub;
    span.set_attribute(Key::from("user"), Value::from(name.to_string()));

    // Open a DB connection
    let db_conn = state.db_conn.load();
    let db = db_conn.as_ref();

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
            if let Some(mut model) = conn {

                // Convert request payload to `ActiveModel`
                let mut name_active_model = patient::name::ActiveModel {
                    id: Set(model.name_id),
                    ..Default::default()
                };
                //set_if_some!(name_active_model, first, payload.name.first);
                let cloned_payload = payload.clone();
                if let Some(name) = payload.name {
                    //if let Some(v) = name.first {
                    //    name_active_model.first = Set(v);
                    //}    
                    if name.first.is_some() {
                        return Err(reject_immutable_field(&span, &patient_id, &cloned_payload, "name.first is immutable"));
                    }
                        //let code = StatusCode::BAD_REQUEST;
                        //span.set_attribute(
                        //    Key::from("http.status_code"),
                        //    Value::from(code.as_u16() as i64),
                        //);
                        //span.set_attribute(
                        //    Key::from("request.payload"),
                        //    Value::from(format!("{:?}", &patient_id)),
                        //);
                        //return Err(AppError(code, anyhow!("Error: name.first is immutable")));
                    //}
                    if let Some(v) = name.middle {
                        name_active_model.middle = Set(v);
                    }    
                    //if let Some(v) = name.surname {
                    //    name_active_model.surname = Set(v);
                    //}
                    if name.surname.is_some() {
                        return Err(reject_immutable_field(&span, &patient_id, &cloned_payload, "name.surname is immutable"));
                    }
                        //let code = StatusCode::BAD_REQUEST;
                        //span.set_attribute(
                        //    Key::from("http.status_code"),
                        //    Value::from(code.as_u16() as i64),
                        //);
                        //span.set_attribute(
                        //    Key::from("request.payload"),
                        //    Value::from(format!("{:?}", &patient_id)),
                        //);
                        //return Err(AppError(code, anyhow!("Error: name.surname is immutable")));
                    //}

                }

                let mut address_active_model = patient::address::ActiveModel {
                    id: Set(model.address_id),
                    ..Default::default()
                };
                if let Some(address) = payload.address {
                    if let Some(v) = address.address_lines {
                        address_active_model.address_lines = Set(v);
                    }
                    if let Some(v) = address.sublocality {
                        address_active_model.sublocality = Set(v);
                    }
                    if let Some(v) = address.locality {
                        address_active_model.locality = Set(v);
                    }
                    if let Some(v) = address.administrative_area {
                        address_active_model.administrative_area = Set(v);
                    }
                    if let Some(v) = address.postal_code {
                        address_active_model.postal_code = Set(v);
                    }
                    if let Some(v) = address.country_region {
                        address_active_model.country_region = Set(v);
                    }
                }

                //let mut birthdate_active_model = patient::birthdate::ActiveModel {
                let birthdate_active_model = patient::birthdate::ActiveModel {
                    id: Set(model.birthdate_id),
                    ..Default::default()
                };
                if payload.birthdate.is_some() {
                    return Err(reject_immutable_field(&span, &patient_id, &cloned_payload, "birthdate is immutable"));
                    //let code = StatusCode::BAD_REQUEST;
                    //span.set_attribute(
                    //    Key::from("http.status_code"),
                    //    Value::from(code.as_u16() as i64),
                    //);
                    //span.set_attribute(
                    //    Key::from("request.payload"),
                    //    Value::from(format!("{:?}", &patient_id)),
                    //);
                    //return Err(AppError(code, anyhow!("Error: birthdate is immutable")));
                }
                //if let Some(birth_date) = payload.birth_date {
                //    if let Some(v) = birth_date.year {
                //        birthdate_active_model.year = Set(v);
                //    }
                //    if let Some(v) = birth_date.month {
                //        birthdate_active_model.month = Set(v);
                //    }
                //    if let Some(v) = birth_date.day {
                //        birthdate_active_model.day = Set(v);
                //    }
                //}
                
                // Stores Models
                let name_model: patient::name::Model = name_active_model.update(db).await?;
                let address_model: patient::address::Model = address_active_model.update(db).await?;
                let birthdate_model: patient::birthdate::Model = birthdate_active_model.update(db).await?;

                // Create and store the full patient record
                let patient_active_model = patient::ActiveModel {
                    id: Set(model.id),
                    name_id: Set(name_model.id),
                    address_id: Set(address_model.id),
                    //birthdate_id: Set(birthdate_model.id),
                    ..Default::default()
                };

                model = patient_active_model.update(db).await?;

                // Constructs response from generated models
                let response_data = Patient {
                    created_at: model.created_at.to_string(),
                    patient_id: model.patient_id.to_string(),
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

fn reject_immutable_field(
    span: &Span,
    patient_id: &Uuid,
    payload: &UpdatePatientRequest,
    message: &str,
) -> AppError {
    let code = StatusCode::BAD_REQUEST;
    span.set_attribute(Key::from("http.status_code"), Value::from(code.as_u16() as i64));
    span.set_attribute(Key::from("request.payload"), Value::from(format!("patient ID: {:?}\n{:?}", patient_id, payload)));
    AppError(code, anyhow!("{message}"))
}
