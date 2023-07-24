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
    let mut cmd = cmd("GET");
    cmd.arg(req_text);

    let redis_val = match connection_manager.send_packed_command(&cmd).await {
        Err(_) => return None,
        Ok(res) => res,
    };
    let redis_res: Result<String, RedisError> = FromRedisValue::from_redis_value(&redis_val);

    match redis_res {
        Ok(cache_hit) => {
            let src_lang = Language::from_str(cache_hit.chars().take(2).collect::<String>().as_str()).unwrap();
            let trans_text = cache_hit.chars().skip(2).collect::<String>();

            let cache_hit = CacheHit::new(src_lang, trans_text);

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
    let cache_val_og: String;
    let cache_val_trans: String ;

    match source_language {
        Language::EN => {
            cache_val_og = format!("EN{}", res_text);
            cache_val_trans = format!("JA{}", req_text);
        },
        Language::JA => {
            cache_val_og = format!("JA{}", res_text);
            cache_val_trans = format!("EN{}", req_text);
        }
    };

    let mut set_og_cmd = cmd("SET");
    set_og_cmd.arg(req_text).arg(cache_val_og);

    let mut set_trans_cmd = cmd("SET");
    set_trans_cmd.arg(res_text).arg(cache_val_trans);

    let mut expire_og = cmd("EXPIRE");
    expire_og.arg(10000);

    let mut expire_trans = cmd("EXPIRE");
    expire_trans.arg(10000);

    let mut pipeline = Pipeline::new();
    pipeline.add_command(set_og_cmd);
    pipeline.add_command(set_trans_cmd);
    pipeline.add_command(expire_og);
    pipeline.add_command(expire_trans);

    let _ = connection_manager.send_packed_commands(&pipeline, 0, 4).await?;

    Ok(())
}
