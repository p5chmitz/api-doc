use serde::Serialize;
//use crate::api::response::error;
use utoipa::ToSchema;

//#[derive(Serialize)]
//pub struct LoginResponse {
//    pub status: String,
//    pub token: String,
//}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    //pub timestamp: String,
    pub token: String,
}
