use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct NameData {
    #[schema(example = "John")]
    pub first: String,

    #[schema(example = "R.")]
    pub middle: String,

    #[schema(example = "Smith")]
    pub surname: String,
}

#[derive(Serialize, ToSchema)]
pub struct AddressData {
    #[schema(example = json!(["123 Fake St.", "suite 1300"]))]
    pub address_lines: Vec<String>,

    #[schema(example = "Brooklyn")]
    pub sublocality: String,

    #[schema(example = "Portland")]
    pub locality: String,

    #[schema(example = "OR")]
    pub administrative_area: String,

    #[schema(example = "97211")]
    pub postal_code: String,

    #[schema(example = "US")]
    pub country_region: String,
}

#[derive(Serialize, ToSchema)]
pub struct BirthdateData {
    #[schema(example = "6")]
    pub day: i32,

    #[schema(example = "8")]
    pub month: i32,

    #[schema(example = "1997")]
    pub year: i32,
}

#[derive(Serialize, ToSchema)]
pub struct Patient {
    /// A system-generated UUID v4 that represents the patient record
    #[schema(example = "3973ebb8-11e5-4725-93b7-3b752caad60f")]
    // utoipa doesn't recognize UUID, so this has to be String
    // to show correctly in the docs; The function handler
    // ensures type safety
    pub patient_id: String,

    /// A system-generated, RFC3339-formatted UTC timestamp
    #[schema(example = "2025-04-01T04:11:48.630391+00:00")]
    pub created_at: String,

    pub name: NameData,
    pub address: AddressData,
    pub birthdate: BirthdateData,
}

#[derive(Serialize, ToSchema)]
pub struct ListPatientsResponse {
    pub patients: Vec<Patient>,
}
