use redis::aio::ConnectionManager;
use sqlite::ConnectionWithFullMutex;

pub struct AppState {
    pub cache_connection: ConnectionManager,
    pub database_connection: ConnectionWithFullMutex
}

impl AppState {
    pub fn new(cache_connection: ConnectionManager, database_connection: ConnectionWithFullMutex) -> AppState {
        AppState { 
            cache_connection,
            database_connection
        }
    }
}
