use std::error::Error;
use redis::{aio::ConnectionManager, cmd, Client, FromRedisValue, RedisError, RedisResult, Pipeline};
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

pub async fn set_cache(
    req_text: &str,
    res_text: &str,
    mut connection_manager: ConnectionManager
) -> Result<(), RedisError> {
    let mut set_original_command = cmd("SET");
    set_original_command.arg(req_text).arg(res_text);

    let mut set_translated_command = cmd("SET");
    set_translated_command.arg(res_text).arg(req_text);

    let mut expire_original = cmd("EXPIRE");
    expire_original.arg(10000);

    let mut expire_translated = cmd("EXPIRE");
    expire_translated.arg(10000);

    let mut pipeline = Pipeline::new();
    pipeline.add_command(set_original_command);
    pipeline.add_command(set_translated_command);
    pipeline.add_command(expire_original);
    pipeline.add_command(expire_translated);

    let _ = connection_manager.send_packed_commands(&pipeline, 0, 4).await;

    Ok(())
}
