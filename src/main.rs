use std::net::SocketAddr;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(greet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet() -> &'static str {
    "Hello you!"
}
