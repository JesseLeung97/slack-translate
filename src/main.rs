use crate::fileserver::file_handler;
use crate::translate::receive_translation_request;
use axum::{
    routing::{get, post},
    Router,
};
use std::{error::Error, net::SocketAddr};
mod deepl;
mod fileserver;
mod models;
mod slackbot;
mod translate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .nest_service("/", get(file_handler))
        .route("/translate", post(receive_translation_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
