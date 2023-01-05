use axum::{
    routing::{get, IntoMakeService},
    Router,
};

use crate::pool::AppPool;

pub async fn init_route() -> IntoMakeService<Router<()>> {
    let route = Router::new().route("/", get(hello));

    let state = AppPool::new().await;

    route.with_state(state).into_make_service()
}

async fn hello() -> &'static str {
    "Hello, World!"
}

// async fn upload_file(mut multipart: Multipart) -> Result<impl IntoResponse> {
//     while let Some(field) = multipart.next_field().await? {
//         let file_name = field.file_name().ok_or(())?.to_string();
//         let data = field.bytes().await?;
//         let mut file = File::create(&file_name).await?;
//         file.write_all(&data).await?;
//     }
//     Ok((StatusCode::OK, "ok"))
// }

// pub async fn hello(PostgresPool(pg): PostgresPool, req: Json<Req>) -> Result<impl IntoResponse> {}
// pub async fn hello(RedisPool(mut rds): RedisPool, claims: Claims<String>, HttpClientPool(client): HttpClientPool,) -> Result<impl IntoResponse> {}
// pub async fn hello(PostgresPool(pg): PostgresPool, Path(id): Path<i64>) -> Result<impl IntoResponse> {}
