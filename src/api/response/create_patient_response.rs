use serde::Serialize;
use crate::entities::patient::Model;

#[derive(Serialize)]
pub struct CreatePatientResponse {
    pub status: String,
    pub data: Model,
}
