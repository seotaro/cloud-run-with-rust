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

    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn decode_base64_to_struct(encoded: &str) -> Result<DATA_JSON_API_V1, String> {
    let decoded_bytes = base64::decode(encoded).map_err(|e| e.to_string())?;
    let decoded_str = String::from_utf8(decoded_bytes).map_err(|e| e.to_string())?;
    serde_json::from_str(&decoded_str).map_err(|e| e.to_string())
}

async fn handle_get_root() -> StatusCode {
    println!("GET");
    StatusCode::OK
}

async fn handle_post_root(input: axum::extract::Json<Payload>) -> StatusCode {
    println!("POST {:?}", input.0.message);
    match decode_base64_to_struct(&input.0.message.data) {
        Ok(data) => {
            println!("Decoded data: {:?} {:?}", data.bucket, data.name);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    StatusCode::OK
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
