use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{routing::get, Router, http::StatusCode};
use tokio::net::TcpListener;


pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/health", get(health_check));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}

#[tokio::test]
async fn test_health_check() {
    let response = health_check().await;
    assert_eq!(response, StatusCode::OK);
}
