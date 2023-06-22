use std::error::Error;
use sqlite::ConnectionWithFullMutex;
use crate::models::Language;
use crate::database::get_users;

pub fn get_analytics(database_connection: &ConnectionWithFullMutex) -> Result<(), Box<dyn Error>> {
    let users = get_users(&database_connection)?;

    // let user = parse_user(user_id).unwrap();

    // let _ = increment_user_count(user_id, connection_manager.clone()).await?;
    // let _ = increment_language_count(source_language, connection_manager.clone()).await?;
    // let _ = increment_total_count(connection_manager.clone()).await?;

    // let _ = append_to_translation_log(&user, source_language, message, translation)?;
    
    Ok(())
}