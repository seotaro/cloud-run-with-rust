use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handle_root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

async fn handle_root(input: axum::extract::Json<InputData>) -> String {
    format!("Received: {:?}", input.0)
}

#[derive(Deserialize, Debug)]
struct InputData {
    name: String,
    age: u32,
}
