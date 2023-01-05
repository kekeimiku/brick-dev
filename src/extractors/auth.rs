use crate::{
    config::CONFIG,
    error::{ApiError, Result},
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let key = CONFIG.jwt_secret.as_bytes();
    Keys {
        encoding: EncodingKey::from_secret(key),
        decoding: DecodingKey::from_secret(key),
    }
});

#[derive(Serialize, Deserialize)]
pub enum JwtType {
    User,
    Manager,
}

#[derive(Serialize, Deserialize)]
pub struct Claims<T> {
    sub: T,
    company: T,
    exp: u64,
    pub node_id: i64,
    pub phone: T,
    pub jwt_type: JwtType,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims<String>
where
    S: Send + Sync,
{
    type Rejection = ApiError;
    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bear = req
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(ApiError::internal_error)?;
        Ok(decode::<Claims<String>>(bear.token(), &KEYS.decoding, &Validation::new(Algorithm::HS256))?.claims)
    }
}

pub fn gen_token(node_id: i64, phone: &str, jwt_type: JwtType) -> Result<String> {
    let time = get_timestamp()?;
    let claims = Claims {
        sub: CONFIG.jwt_sub.as_str(),
        company: CONFIG.jwt_company.as_str(),
        exp: time + CONFIG.jwt_exp,
        node_id,
        phone,
        jwt_type,
    };
    Ok(encode(&Header::default(), &claims, &KEYS.encoding)?)
}

pub fn get_timestamp() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}
