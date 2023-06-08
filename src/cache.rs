use redis::{aio::ConnectionManager, FromRedisValue, RedisError};

pub fn get_redis_connection_manager() -> redis::RedisResult<ConnectionManager> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let connection_manager = ConnectionManager::new(client);

    Ok(connection_manager)
}

pub fn check_cache(req_text: &str, mut connection_manager: ConnectionManager) -> Option<String> {
    let cmd = redis::pipe().cmd("GET").arg(req_text);
    let redis_value = connection_manager.send_packed_command(cmd).await?;
    let redis_response: Result<String, RedisError> =
        redis::FromRedisValue::from_redis_value(&redis_value);

    match redis_response {
        Ok(cache_hit) => return Some(cache_hit),
        Err(_) => return None,
    }
}
