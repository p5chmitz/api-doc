use crate::api::response::error::AppError;
use crate::api::response::list_patients::{
    AddressData, BirthdateData, ListPatientsResponse, NameData, Patient,
};
use crate::api::response::TokenClaims;
use crate::entities::patient::{self, address, birthdate, name};
use crate::state::ApplicationState;
use anyhow::anyhow;
use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use std::sync::Arc;

#[derive(serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct GetPatientQuery {
    #[schema(example = "Jane")]
    pub first_name: Option<String>,

    #[schema(example = "Doe")]
    pub surname: Option<String>,

    #[schema(example = "1974")]
    pub birth_year: Option<i32>,
}

/// Returns a list of patients based on optional query parameters
#[utoipa::path(
    get,
    path = "/patient",
    params(GetPatientQuery),
    tag = "Patients",
    responses(
        (status = 200, description = "Success", body = ListPatientsResponse),
        (status = 400, description = "Generic error response format", body = ErrorResponse),
    ),
    security(
        ("api_jwt_token" = [])
    )
)]
#[debug_handler]
pub async fn list(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Query(query): Query<GetPatientQuery>,
) -> Result<Json<ListPatientsResponse>, AppError> {
    // Create a DB connection binding to share it
    let db_conn = state.db_conn.load();
    let db = db_conn.as_ref();

    // Start building the query
    let mut query_builder = patient::Entity::find();
    query_builder = query_builder
        .join(JoinType::InnerJoin, patient::Relation::Name.def())
        .join(JoinType::InnerJoin, patient::Relation::Birthdate.def());

    // Add filters if query parameters are present
    if let Some(first) = &query.first_name {
        query_builder = query_builder.filter(name::Column::First.eq(first));
    }
    if let Some(surname) = &query.surname {
        query_builder = query_builder.filter(name::Column::Surname.eq(surname));
    }
    if let Some(year) = &query.birth_year {
        query_builder = query_builder.filter(birthdate::Column::Year.eq(*year));
    }

    // Execute the query and get the patients
    let patient_models = query_builder.all(db).await;

    let mut response_vec = Vec::new();

    match patient_models {
        // await returns Result<Option<Model>, DbErr>
        // so you have to safely unwrap all its thorny layers
        Ok(conn) => {
            // Fetches data for each match and pushes to response vec
            for model in conn {
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

                // Construct the Patient
                let patient = Patient {
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

                response_vec.push(patient);
            }

            return Ok(Json(ListPatientsResponse { patients: response_vec }));
        }
        // If the search is not Ok, issue a generic DB
        // connection error and obfuscate the specifics
        Err(_) => {
            return Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow!("Uh oh..."),
            ))
        }
    }
}
