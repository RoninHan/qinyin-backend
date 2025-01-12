use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use service::UserServices;
use thiserror::Error;

use crate::tools::AppState;

#[derive(Debug, Error)]
pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct Auth {
    // 结构体内容
}

impl Auth {
    pub async fn authorization_middleware(
        state: State<AppState>,
        mut req: Request,
        next: Next,
    ) -> Response {
        let auth_header = req.headers_mut().get(axum::http::header::AUTHORIZATION);
        // auth_header 轉字符串
        let auth_header = match auth_header {
            Some(header) => header.to_str().unwrap(),
            None => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(axum::body::Body::from("No authorization header provided"))
                    .unwrap();
            }
        };
        let mut header = auth_header.split_whitespace();
        let (bearer, token) = (header.next(), header.next());
        let token_data = Self::decode_jwt(token.unwrap().to_string());
        let current_user =
            UserServices::find_user_by_email(&state.conn, &token_data.unwrap().claims.email).await;
        match current_user {
            Ok(Some(user)) => {
                req.extensions_mut().insert(user);
            }
            Ok(None) => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(axum::body::Body::from("User not found"))
                    .unwrap();
            }
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(axum::body::Body::from("Internal server error"))
                    .unwrap();
            }
        }
        let response = next.run(req).await;
        response
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        let hash = hash(password, DEFAULT_COST)?;
        Ok(hash)
    }

    pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
        let secret: String = "randomStringTypicallyFromEnv".to_string();
        let now = Utc::now();
        let expire: chrono::TimeDelta = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        let claim = Claims { iat, exp, email };

        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
        let secret = "randomStringTypicallyFromEnv".to_string();
        let result: Result<TokenData<Claims>, StatusCode> = decode(
            &jwt_token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
        result
    }
}

#[derive(Serialize, Deserialize)]
// Define a structure for holding claims data used in JWT tokens
pub struct Claims {
    pub exp: usize,    // Expiry time of the token
    pub iat: usize,    // Issued at time of the token
    pub email: String, // Email associated with the token
}
