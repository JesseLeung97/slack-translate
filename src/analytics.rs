use redis::aio::ConnectionManager;
use crate::models::parse_user;
use crate::cache::increment_user_count;

pub fn update_analytics(mut connection_manager: ConnectionManager, user_id: &str, message: &str, translation: &str) -> Result<Ok, Box<dyn Error>> {
    let user = parse_user(user_id).unwrap();

    increment_user_count(user_id)?;
    
    Ok(())
}