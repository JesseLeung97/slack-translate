use redis::{aio::ConnectionManager, cmd, Client, FromRedisValue, RedisError, RedisResult};
use crate::models::Language;

pub async fn get_redis_connection_manager() -> RedisResult<ConnectionManager> {
    let client = Client::open("redis://localhost:6380")?;
    let connection_manager = ConnectionManager::new(client).await?;

    Ok(connection_manager)
}

pub async fn check_cache(
    req_text: &str,
    mut connection_manager: ConnectionManager,
) -> Option<String> {
    let mut command = cmd("GET");
    command.arg(req_text);

    let redis_value = match connection_manager.send_packed_command(&command).await {
        Err(_) => return None,
        Ok(res) => res,
    };
    let redis_response: Result<String, RedisError> = FromRedisValue::from_redis_value(&redis_value);

    match redis_response {
        Ok(cache_hit) => return Some(cache_hit),
        Err(_) => return None,
    }
}
