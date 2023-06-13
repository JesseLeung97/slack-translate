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
    let command = command.arg(req_text);

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

pub async fn increment_user_count(
    user_id: &str,
    mut connection_manager: ConnectionManager
) -> Result<usize, RedisError> {
    let mut command = cmd("INCR");
    let command = command.arg(user_id);

    let redis_value: redis::Value = connection_manager.send_packed_command(&command).await?;
    let user_translation_count: usize = FromRedisValue::from_redis_value(&redis_value)?;

    Ok(user_translation_count)
}

pub async fn increment_language_count(
    language: Language,
    mut connection_manager: ConnectionManager
) -> Result<usize, RedisError> {
    let mut command = cmd("INCR");
    let command = command.arg(language.to_string());

    let redis_value = connection_manager.send_packed_command(&command).await?;
    let language_translation_count: usize = FromRedisValue::from_redis_value(&redis_value)?;

    Ok(language_translation_count)
}

pub async fn increment_total_count(
    mut connection_manager: ConnectionManager
) -> Result<usize, RedisError> {
    let mut command = cmd("INCR");
    let command = command.arg("TOTAL");

    let redis_value = connection_manager.send_packed_command(&command).await?;
    let total_count: usize = FromRedisValue::from_redis_value(&redis_value)?;

    Ok(total_count)
}
