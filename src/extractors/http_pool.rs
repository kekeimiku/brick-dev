use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use reqwest::Client;

use crate::{error::ApiError, pool::AppPool};

pub type HttpClient = Client;
impl FromRef<AppPool> for Client {
    fn from_ref(input: &AppPool) -> Self {
        input.http_client.clone()
    }
}

pub struct HttpClientPool(pub Client);

#[async_trait]
impl<S> FromRequestParts<S> for HttpClientPool
where
    Client: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = HttpClient::from_ref(state);
        Ok(Self(pool))
    }
}

pub fn init_http_client() -> Client {
    reqwest::Client::new()
}
