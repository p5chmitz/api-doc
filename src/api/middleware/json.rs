//use crate::api::response::error::Status;
use crate::api::response::error::ErrorResponse;
use axum::http::Request;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    Json,
};
use serde_json::Value;

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for CustomJson<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let error_response = ErrorResponse {
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                    reason: "Invalid JSON",
                    message: rejection.body_text(),
                };

                // Serialize the error struct into a map that preserves the order
                let error_map = serde_json::to_value(error_response)
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .clone();

                // Return the error response with the preserved order
                Err((rejection.status(), Json(Value::Object(error_map))))
            }
        }
    }
}
