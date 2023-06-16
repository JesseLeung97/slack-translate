use std::error::Error;
use sqlite::{Connection, ConnectionWithFullMutex,};
use crate::models::Language;

pub fn get_database_connection_manager() -> Result<ConnectionWithFullMutex, Box<dyn Error>> {
    let connection = Connection::open_with_full_mutex("database/slack-translate_production.sqlite")?;

    Ok(connection)
}

pub fn append_to_translation_log(
    user_id: &str,
    language: &Language, 
    message: &str, 
    translation: &str,
    database_connection: &ConnectionWithFullMutex
) -> Result<(), Box<dyn Error>> {
    let insert_query = format!("
        INSERT INTO translation_log (user_id, language, original_text, translated_text) VALUES (\"{}\", \"{}\", \"{}\",\"{}\")",
        user_id, language.to_string(), message, translation
    );

    database_connection.execute(insert_query)?;

    Ok(())
}