use crate::api::middleware::json::CustomJson;
use crate::api::request::login_request::LoginRequest;
use crate::api::response::error::AppError;
use crate::api::response::login_response::LoginResponse;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

use anyhow::anyhow;
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::instrument;
//use tracing::{Level, Span};
use opentelemetry::{Key, Value};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::entities::user;

/// Get a JWT
#[utoipa::path(
    post,
    path = "/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Success", body = LoginResponse),
        (status = 400, description = "Generic error response format", body = ErrorResponse),
    ),
)]
#[instrument(level = "info", name = "login", skip_all)]
pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    CustomJson(payload): CustomJson<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Start a tracing span
    let span = Span::current();
    span.set_attribute(Key::from("http.method"), Value::from("POST"));

    // Validate that the password is correct
    match user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        // NOTE: EntityTrait::find().all() returns a list
        //.all(state.db_conn.load().as_ref())
        .one(state.db_conn.load().as_ref())
        .await
    {
        Ok(admins) => {
            // Adds the username to the trace
            let name = &payload.username;
            span.set_attribute(Key::from("user"), Value::from(name.to_string()));

            // The user wasn't found
            if admins.is_none() {
                //span.record("payload", &tracing::field::debug(&payload));
                let response: AppError =
                    AppError(StatusCode::UNAUTHORIZED, anyhow!("User doesn't exist"));
                span.set_attribute(
                    Key::from("request.payload"),
                    Value::from(format!("{:?}", &payload)),
                );
                span.set_attribute(
                    Key::from("response.payload"),
                    Value::from(format!("{:?}", &response)),
                );
                span.set_attribute(Key::from("http.status_code"), Value::from(401));
                return Err(response);
            }

            // The password doesn't match
            if validate_password(&payload.password, &admins.unwrap().password).is_err() {
                let response: AppError =
                    AppError(StatusCode::UNAUTHORIZED, anyhow!("Invalid password"));
                span.set_attribute(
                    Key::from("request.payload"),
                    Value::from(format!("{:?}", &payload)),
                );
                span.set_attribute(
                    Key::from("response.payload"),
                    Value::from(format!("{:?}", &response)),
                );
                span.set_attribute(Key::from("http.status_code"), Value::from(401));
                return Err(response);
            }
        }
        // Something went wrong on the client side
        Err(_) => {
            span.set_attribute(Key::from("http.status_code"), Value::from(500));
            return Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                anyhow!("We fucked up"),
            ));
        }
    }

    // If validation doesn't error, issue the token
    let secret = &state.settings.load().token_secret;
    let timeout = state.settings.load().token_timeout_seconds;

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    //let exp =
    //  (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let exp = (now + chrono::Duration::seconds(timeout)).timestamp() as usize;
    let claims = TokenClaims {
        sub: payload.username,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or("".to_string());

    let response = LoginResponse { token };

    span.set_attribute(Key::from("http.status_code"), Value::from(200));

    //tracing::info!("Login span");

    Ok(Json(response))
}

fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow!(e.to_string()))?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_e| anyhow!("Failed to verify password"))?;

    Ok(())
}
