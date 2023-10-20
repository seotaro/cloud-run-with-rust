mod pubsub;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_get_root))
        .route("/", post(handle_post_root));

    let port: u16 = std::env::var("PORT")
        .and_then(|p| p.parse().map_err(|_| std::env::VarError::NotPresent))
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_get_root() -> StatusCode {
    println!("GET");

    StatusCode::OK
}

async fn handle_post_root(input: axum::extract::Json<pubsub::Payload>) -> StatusCode {
    println!("POST {:?}", input.0.message);

    match pubsub::parse_data::<pubsub::DataJsonApiV1>(&input.0.message.data) {
        Ok(data) => {
            println!("Decode data: {:?}", data);
        }
        Err(e) => {
            println!("Decode error: {}", e);
        }
    }

    StatusCode::OK
}
