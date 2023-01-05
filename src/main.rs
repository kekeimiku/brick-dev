use brick_dev::{config::CONFIG, router};

use std::{net::SocketAddr, str::FromStr};

#[tokio::main]
async fn main() {
    let app = router::init_route().await;
    let addr = SocketAddr::from_str(&CONFIG.socket_addr).expect("invalid socket address");

    axum::Server::bind(&addr).serve(app).await.unwrap();
}
