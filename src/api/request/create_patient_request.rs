//use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Name {
    pub first: String,
    pub middle: Option<String>,
    pub surname: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub address_lines: Vec<String>,
    pub sublocality: Option<String>,
    pub locality: Option<String>,
    pub administrative_area: Option<String>,
    pub postal_code: Option<String>,
    pub country_region: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthDate {
    pub day: i32,
    pub month: i32,
    pub year: i32,
}
//impl BirthDate {
//    pub fn _to_naive_date(&self) -> Option<NaiveDate> {
//        NaiveDate::from_ymd_opt(self.year, self.month.into(), self.day.into())
//    }
//}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatePatientRequest {
    pub name: Name,
    pub address: Address,
    pub birth_date: BirthDate,
}
