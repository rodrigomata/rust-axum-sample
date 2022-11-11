use axum::{routing::get, Router, Server};
use std::net::SocketAddr;
use std::net::TcpListener;

pub async fn run(port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    Server::bind(&addr)
        .serve(router().into_make_service())
        .await
        .unwrap();
}

pub fn router() -> Router {
    Router::new().route("/", get(health_check))
}

pub fn get_available_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {port}");
    port
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
