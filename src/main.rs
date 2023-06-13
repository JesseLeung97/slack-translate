use crate::cache::get_redis_connection_manager;
use crate::fileserver::file_handler;
use crate::models::AppState;
use crate::translate::receive_translation_request;
use axum::{
    routing::{get, post},
    Router,
};
use std::{error::Error, net::SocketAddr};
mod cache;
mod deepl;
mod fileserver;
mod filewriter;
mod models;
mod slackbot;
mod translate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let redis_connection = get_redis_connection_manager().await?;
    let app_state = AppState::new(redis_connection);

    let app = Router::new()
        .nest_service("/", get(file_handler))
        .route("/translate", post(receive_translation_request))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
