use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub connection_manager: ConnectionManager,
}

impl AppState {
    pub fn new(connection_manager: ConnectionManager) -> AppState {
        AppState { connection_manager }
    }
}
