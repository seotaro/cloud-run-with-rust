use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

use base64;
use serde::Deserialize;
use serde_json;
use std::env;
use std::net::SocketAddr;
use std::str;

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

async fn handle_post_root(input: axum::extract::Json<Payload>) -> StatusCode {
    println!("POST {:?}", input.0.message);

    match decode_data_v1(&input.0.message.data) {
        Ok(data) => {
            println!("Decode data: {:?}", data);
        }
        Err(e) => {
            println!("Decode error: {}", e);
        }
    }

    StatusCode::OK
}

fn decode_data_v1(src: &str) -> Result<DATA_JSON_API_V1, String> {
    let dest_bytes = base64::decode(src).map_err(|e| e.to_string())?;
    let dest_str = String::from_utf8(dest_bytes).map_err(|e| e.to_string())?;
    serde_json::from_str(&dest_str).map_err(|e| e.to_string())
}

#[derive(Deserialize, Debug)]
struct Payload {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Attributes {}

#[derive(Deserialize, Debug)]
struct Message {
    attributes: Attributes,
    data: String,
    messageId: String,
    message_id: String,
    publishTime: String,
    publish_time: String,
}

#[derive(Deserialize, Debug)]
struct DATA_JSON_API_V1 {
    kind: String,
    id: String,
    selfLink: String,
    name: String,
    bucket: String,
    generation: String,
    metageneration: String,
    contentType: String,
    timeCreated: String,
    updated: String,
    storageClass: String,
    timeStorageClassUpdated: String,
    size: String,
    md5Hash: String,
    mediaLink: String,
    crc32c: String,
    etag: String,
}
