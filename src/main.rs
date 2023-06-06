use axum::{http::StatusCode, routing::{get, post}, Router};
use std::{net::SocketAddr, error::Error};
mod slackbot;
mod deepl;
mod models;
mod translate;
use crate::translate::receive_translation_request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(handler))
        .route("/translate", post(receive_translation_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler() -> (StatusCode, String) {
    println!("Hello, world!");
    (StatusCode::OK, String::from("Hello, World!"))
}