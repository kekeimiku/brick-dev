pub mod postgres {
    use axum::{
        async_trait,
        extract::{FromRef, FromRequestParts},
        http::request::Parts,
    };

    use bb8_postgres::{
        bb8::{Pool, PooledConnection},
        tokio_postgres::{Error, NoTls},
        PostgresConnectionManager,
    };

    use crate::{error::ApiError, pool::AppPool};

    pub type PostgresConnectionPool = Pool<PostgresConnectionManager<NoTls>>;
    impl FromRef<AppPool> for PostgresConnectionPool {
        fn from_ref(input: &AppPool) -> Self {
            input.postgres.clone()
        }
    }

    pub async fn init_postgres(info: &str) -> Result<PostgresConnectionPool, Error> {
        let manager = PostgresConnectionManager::new_from_stringlike(info, NoTls)?;
        Pool::builder().build(manager).await
    }

    pub struct PostgresPool(pub PooledConnection<'static, PostgresConnectionManager<NoTls>>);

    #[async_trait]
    impl<S> FromRequestParts<S> for PostgresPool
    where
        PostgresConnectionPool: FromRef<S>,
        S: Send + Sync,
    {
        type Rejection = ApiError;

        async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
            let pool = PostgresConnectionPool::from_ref(state);
            let postgres = pool.get_owned().await.map_err(ApiError::internal_error)?;
            Ok(Self(postgres))
        }
    }
}

pub mod reids {
    use axum::{
        async_trait,
        extract::{FromRef, FromRequestParts},
        http::request::Parts,
    };

    use bb8_redis::{
        bb8::{Pool, PooledConnection},
        redis::RedisError,
        RedisConnectionManager,
    };

    use crate::{error::ApiError, pool::AppPool};

    pub type RedisConnectionPool = Pool<RedisConnectionManager>;
    impl FromRef<AppPool> for RedisConnectionPool {
        fn from_ref(input: &AppPool) -> Self {
            input.redis.clone()
        }
    }

    pub async fn init_redis(info: &str) -> Result<RedisConnectionPool, RedisError> {
        let manager = RedisConnectionManager::new(info)?;
        Pool::builder().build(manager).await
    }

    pub struct RedisPool(pub PooledConnection<'static, RedisConnectionManager>);

    #[async_trait]
    impl<S> FromRequestParts<S> for RedisPool
    where
        RedisConnectionPool: FromRef<S>,
        S: Send + Sync,
    {
        type Rejection = ApiError;

        async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
            let pool = RedisConnectionPool::from_ref(state);
            let redis = pool.get_owned().await.map_err(ApiError::internal_error)?;
            Ok(Self(redis))
        }
    }
}
