use crate::api::request::login::LoginRequest;
use crate::api::response::login::LoginResponse;
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

use crate::entities::user;

pub async fn login(
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Validate that the password is correct
    match user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        // NOTE: EntityTrait::find().all() returns a list
        //.all(state.db_conn.load().as_ref())
        .one(state.db_conn.load().as_ref())
        .await
    {
        Ok(admins) => {
            // The user wasn't found
            if admins.is_none() {
                return Err(StatusCode::UNAUTHORIZED);
            }
            //if admins.is_empty() {
            //    return Err(StatusCode::UNAUTHORIZED)
            //}

            // The password doesn't match
            //let admin = &admins[0];
            if validate_password(&payload.password, &admins.unwrap().password)
                //if validate_password(&payload.password, &admin.password)
                .is_err()
            {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        // Something went wrong on the client side
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
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
    .unwrap();

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };

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
