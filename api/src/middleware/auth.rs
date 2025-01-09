use axum::{
    body::Body,
    extract::{Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
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
        State(state): State<AppState>,
        mut req: Request,
        next: Next,
    ) -> Result<Response<Body>, AuthError> {
        let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
        let auth_header = match auth_header {
            Some(header) => header.to_str().map_err(|_| AuthError {
                message: "Empty header is not allowed".to_string(),
                status_code: StatusCode::FORBIDDEN,
            })?,
            None => {
                return Err(AuthError {
                    message: "Please add the JWT token to the header".to_string(),
                    status_code: StatusCode::FORBIDDEN,
                })
            }
        };
        let mut header = auth_header.split_whitespace();
        let (bearer, token) = (header.next(), header.next());
        let token_data = match Self::decode_jwt(token.unwrap().to_string()) {
            Ok(data) => data,
            Err(_) => {
                return Err(AuthError {
                    message: "Unable to decode token".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                })
            }
        };
        // Fetch the user details from the database
        let current_user =
            match UserServices::find_user_by_email(&state.conn, &token_data.claims.email).await {
                Ok(Some(user)) => user,
                Ok(None) => {
                    return Err(AuthError {
                        message: "You are not an authorized user".to_string(),
                        status_code: StatusCode::UNAUTHORIZED,
                    })
                }
                Err(_) => {
                    return Err(AuthError {
                        message: "Database error".to_string(),
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    })
                }
            };
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
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
