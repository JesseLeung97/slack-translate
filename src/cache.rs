use redis::{aio::ConnectionManager, cmd, Client, FromRedisValue, RedisError, RedisResult, Pipeline};
use crate::models::{Language, CacheHit};
use std::str::FromStr;

pub async fn get_redis_connection_manager() -> RedisResult<ConnectionManager> {
    let client = Client::open("redis://127.0.0.1:6380")?;
    let connection_manager = ConnectionManager::new(client).await?;

    Ok(connection_manager)
}

pub async fn check_cache(
    req_text: &str,
    mut connection_manager: ConnectionManager,
) -> Option<CacheHit> {
    let mut command = cmd("GET");
    command.arg(req_text);

    let redis_value = match connection_manager.send_packed_command(&command).await {
        Err(_) => return None,
        Ok(res) => res,
    };
    let redis_response: Result<String, RedisError> = FromRedisValue::from_redis_value(&redis_value);

    match redis_response {
        Ok(cache_hit) => {
            let source_language = Language::from_str(cache_hit.chars().take(2).collect::<String>().as_str()).unwrap();
            let translated_text = cache_hit.chars().skip(2).collect::<String>();

            let cache_hit = CacheHit::new(source_language, translated_text);

            return Some(cache_hit);
        },
        Err(_) => return None,
    }
}

pub async fn set_cache(
    req_text: &str,
    res_text: &str,
    source_language: Language,
    mut connection_manager: ConnectionManager
) -> Result<(), RedisError> {
    let cache_value_original: String;
    let cache_value_translate: String ;

    match source_language {
        Language::EN => {
            cache_value_original = format!("EN{}", res_text);
            cache_value_translate = format!("JA{}", req_text);
        },
        Language::JA => {
            cache_value_original = format!("JA{}", res_text);
            cache_value_translate = format!("EN{}", req_text);
        }
    };

    let mut set_original_command = cmd("SET");
    set_original_command.arg(req_text).arg(cache_value_original);

    let mut set_translated_command = cmd("SET");
    set_translated_command.arg(res_text).arg(cache_value_translate);

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
