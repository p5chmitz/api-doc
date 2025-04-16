use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug)]
pub struct ErrorResponse {
    /// The HTTP status code value
    #[schema(example = "422")]
    pub status_code: u16,

    /// The HTTP status code reason
    #[schema(example = "Unprocessable Entity")]
    pub reason: &'static str,

    /// A contextual message regarding the error
    #[schema(
        example = "Failed to parse the request body as JSON: birth_date.?: expected `,` or `}` at line 12 column 9"
    )]
    pub message: String,
}

//#[derive(Serialize, ToSchema)]
//pub enum Status {
//    Success,
//    _Error,
//}

#[derive(Debug)]
pub struct AppError(pub StatusCode, pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(ErrorResponse {
                //status: Status::Error,
                status_code: self.0.as_u16(),
                reason: self.0.canonical_reason().unwrap_or("Unknown error"),
                message: self.1.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}
