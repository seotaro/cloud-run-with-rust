use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

use std::env;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

async fn root() -> String {
    let mut hello = "Hello ".to_string();
    match env::var("TARGET") {
        Ok(target) => {
            hello.push_str(&target);
        }
        Err(_e) => hello.push_str("World"),
    };
    hello
}
