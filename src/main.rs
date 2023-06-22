use crate::cache::get_redis_connection_manager;
use crate::fileserver::file_handler;
use crate::models::AppState;
use crate::translate::receive_translation_request;
use crate::database::get_database_connection_manager;
use axum::{
    routing::{get, post},
    Router,
    Extension
};
use std::{error::Error, net::SocketAddr, sync::Arc};
mod cache;
mod deepl;
mod fileserver;
mod filewriter;
mod models;
mod slackbot;
mod translate;
mod analytics;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let redis_connection = get_redis_connection_manager().await?;
    let database_connection = get_database_connection_manager()?;
    let app_state = AppState::new(redis_connection, database_connection);

    let app_state = Arc::new(app_state);

    let app = Router::new()
        .nest_service("/", get(file_handler))
        .route("/translate", post(receive_translation_request))
        .layer(Extension(app_state.clone()));
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
