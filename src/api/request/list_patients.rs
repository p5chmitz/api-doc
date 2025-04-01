use serde::Deserialize;
use utoipa::ToSchema;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct BirthDate {
    /// The day for a birth date with no leading zeros
    #[schema(example = "6")]
    pub day: i32,
    
    /// The month for the birth date with no leading zeroes
    #[schema(example = "8")]
    pub month: i32,
    
    /// The year for the birth date in YYYY format
    #[schema(example = "1997")]
    pub year: i32,
}

#[derive(Deserialize, ToSchema)]
pub struct ListPatients {
    /// Patient ID
    #[schema(example = "3973ebb8-11e5-4725-93b7-3b752caad60f")]
    pub patient_id: Option<uuid::Uuid>,

    /// Patien's first name
    #[schema(example = "John")]
    pub first_name: Option<String>,

    /// Patient's surname
    #[schema(example = "Smith")]
    pub surname: Option<String>,

    /// The patient's birthdate
    #[schema(example = json!({
      "day": 6,
      "month": 8,
      "year": 1997
    }))]
    pub birthdate: Option<BirthDate>,
}

