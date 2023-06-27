use std::error::Error;
use sqlite::ConnectionWithFullMutex;
use crate::models::Language;
use crate::database::{ get_translation_log, get_users };

pub fn get_analytics(database_connection: &ConnectionWithFullMutex) -> Result<(), Box<dyn Error>> {
    let users = get_users(&database_connection)?;
    let translation_log = get_translation_log(&database_connection);

    Ok(())
}
