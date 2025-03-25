use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
