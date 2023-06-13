use std::error::Error;
use redis::aio::ConnectionManager;
use crate::models::parse_user;
use crate::cache::{increment_user_count, increment_language_count, increment_total_count};
use crate::models::Language;

pub async fn update_analytics(
    connection_manager: ConnectionManager, 
    user_id: &str,
    language: Language, 
    message: &str, 
    translation: &str
) -> Result<(), Box<dyn Error>> {
    let user = parse_user(user_id).unwrap();

    let _ = increment_user_count(user_id, connection_manager.clone()).await?;
    let _ = increment_language_count(language, connection_manager.clone()).await?;
    let _ = increment_total_count(connection_manager.clone()).await?;


    

    Ok(())
}