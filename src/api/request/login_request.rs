use serde::Deserialize;
use utoipa::ToSchema;

//#[allow(dead_code)]
#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Your username
    #[schema(default = "admin")]
    pub username: String,
    /// Your password
    #[schema(default = "apidocpass")]
    pub password: String,
}
