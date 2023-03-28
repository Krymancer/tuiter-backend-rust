use crate::router::ApiContext;
use axum::{http::{HeaderValue, StatusCode, header::AUTHORIZATION}, Extension, extract::FromRequestParts};
use time::OffsetDateTime;
use jwt::{token::signed::SignWithKey, VerifyWithKey};
use uuid::Uuid;
use hmac::{Hmac, Mac};
use sha2::Sha384;
use anyhow::Error;

use axum::{
    async_trait,
    extract::{FromRequest, FromRef},
    http::{self, Request, request::Parts},
};


const DEFAULT_SESSION_LENGTH : time::Duration = time::Duration::weeks(2);

const SCHEME_PREFIX : &str = "Bearer ";

pub struct AuthUser {
    pub user_id: Uuid,
}

pub struct MaybeUser(pub Option<AuthUser>);

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthUserClaims {
    user_id: Uuid,
    exp: i64,
}

impl AuthUser {
    pub(in crate) fn to_jwt(&self, context: &ApiContext) -> String {
        let hmac = Hmac::<Sha384>::new_from_slice(context.config.hmac_key.as_bytes())
            .expect("HMAC can take key of any size");

        AuthUserClaims {
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC sing should not fail")
    }

    fn from_authorization(context: &ApiContext, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            println!("Failed to parse auth header");
            Error::msg("Unauthorized")
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            println!("Auth header does not start with {}", SCHEME_PREFIX);
            return Err(Error::msg("Unauthorized"));
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];
        
        let jwt = jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token)?;

        let hmac = Hmac::<Sha384>::new_from_slice(context.config.hmac_key.as_bytes())
            .expect("HMAC can take key of any size");

        let jwt = jwt.verify_with_key(&hmac)?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            println!("JWT expired");
            return Err(Error::msg("Unauthorized"));
        }
    
        Ok(Self {
            user_id: claims.user_id,
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where S: Send + Sync,
      ApiContext: FromRef<S>
{
    type Rejection = axum::http::StatusCode;
    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let context : ApiContext = ApiContext::from_ref(state);

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(StatusCode::UNAUTHORIZED)?;

        match Self::from_authorization(&context, auth_header) {
            Ok(user) => Ok(user),
            Err(_) => Err(StatusCode::UNAUTHORIZED)
        }
    }
}
