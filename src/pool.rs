use crate::{
    config::CONFIG,
    extractors::{
        db_pool::{
            postgres::{init_postgres, PostgresConnectionPool},
            reids::{init_redis, RedisConnectionPool},
        },
        http_pool::{init_http_client, HttpClient},
    },
};

#[derive(Clone)]
pub struct AppPool {
    pub postgres: PostgresConnectionPool,
    pub redis: RedisConnectionPool,
    pub http_client: HttpClient,
}

impl AppPool {
    pub async fn new() -> Self {
        Self {
            postgres: init_postgres(&CONFIG.pg_conn)
                .await
                .expect("can connect to postgres"),
            redis: init_redis(&CONFIG.redis_conn)
                .await
                .expect("can connect to redis"),
            http_client: init_http_client(),
        }
    }
}
