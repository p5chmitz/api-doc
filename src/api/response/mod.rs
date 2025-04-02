pub mod create_patient_response;
pub mod error;
pub mod login_response;

// Struct to store token claims for processing
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
