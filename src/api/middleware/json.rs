use crate::api::response::error::ErrorResponse;
use axum::{
    async_trait,
    body::Body,
    extract::{rejection::JsonRejection, FromRequest},
    response::{IntoResponse, Response},
    http::{Request, StatusCode},
    Json,
};
use serde_json::Value;

use opentelemetry::{Key, Value as otelVal};
use serde::de::DeserializeOwned;
use tracing::{instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S, Body> for CustomJson<T>
where
    Json<T>: FromRequest<S, Body, Rejection = JsonRejection>,
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    #[instrument(level = "info", name = "middleware_json_parsing", skip_all)]
    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        // Clones the request method for error tracing
        let method = req.method().clone();

        // Deconstruct the request and buffer the body into bytes
        let (parts, body) = req.into_parts();
        let body_bytes = match hyper::body::to_bytes(body).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": format!("Failed to read body: {}", err) })),
                ));
            }
        };

        // Rebuild the request with the buffered body
        let req = Request::from_parts(parts, Body::from(body_bytes.clone()));

        // Validates JSON
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)), // No span for valid JSON
            Err(rejection) => {
                let status = rejection.status();
                let reason = status.canonical_reason().unwrap_or("Unknown");

                // Only creates a span if theres an error
                let span = Span::current();
                span.set_attribute(
                    Key::from("http.method"),
                    otelVal::from(method.as_str().to_string()),
                );
                let span = Span::current();
                span.set_attribute(
                    Key::from("http.status_code"),
                    otelVal::from(status.as_u16() as i64),
                );
                let body = String::from_utf8_lossy(&body_bytes);
                tracing::info!(
                    request.payload = %body,
                    "Malformed JSON"
                );
                //span.set_attribute(
                //    Key::from("request.payload"),
                //    otelVal::from(format!("{:?}", body)),
                //);
                span.set_attribute(
                    Key::from("response.payload"),
                    otelVal::from(format!("{:?}", &rejection)),
                );

                let error_response = ErrorResponse {
                    status_code: status.as_u16(),
                    reason,
                    message: rejection.body_text(),
                };

                //let error_response = ErrorResponse {
                //    status_code: StatusCode::BAD_REQUEST.as_u16(),
                //    reason: "Invalid JSON",
                //    message: rejection.body_text(),
                //};

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

#[instrument(level = "info", name = "middleware_json_wrapper", skip_all)]
pub fn to_response(res: Json<Value>) -> Response {
    let mut body = res.0.to_string();
    body.push('\n');
    body.into_response()
}
