use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{Response, StatusCode, header::LOCATION, request::Parts},
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::configs;

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub is_admin: bool,
    pub created_at: usize,
    exp: usize,
}

impl JwtClaims {
    pub fn new(is_admin: bool) -> Self {
        let created_at = Self::get_created_at();
        let expiration_time = Self::get_expiration_token();

        JwtClaims {
            is_admin,
            created_at,
            exp: expiration_time,
        }
    }

    fn get_created_at() -> usize {
        let now = SystemTime::now();

        now.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize
    }

    fn get_expiration_token() -> usize {
        let jwt_opts = configs::get_jwt_opts();

        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

        let expiration_time = jwt_opts.expiration;

        duration_since_epoch.as_secs() as usize + expiration_time
    }

    pub fn gen_token(&self) -> String {
        let jwt_opts = configs::get_jwt_opts();

        let secret = jwt_opts.secret.as_bytes();

        encode(&Header::default(), &self, &EncodingKey::from_secret(secret)).unwrap()
    }

    fn parse_token(token: String) -> Result<JwtClaims, impl IntoResponse> {
        let jwt_opts = configs::get_jwt_opts();

        let secret = jwt_opts.secret.as_bytes();

        match decode::<Self>(
            &token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        ) {
            Ok(claim) => Ok(claim.claims),
            Err(_) => Err(Errors::InvalidToken),
        }
    }
}

pub enum Errors {
    InvalidToken,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(LOCATION, "/user/login")
            .body(Body::empty())
            .unwrap();
    }
}

impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = Errors;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let Some(token) = jar.get("token") else {
            return Err(Errors::InvalidToken);
        };

        JwtClaims::parse_token(token.value().to_string()).map_err(|_| Errors::InvalidToken)
    }
}
