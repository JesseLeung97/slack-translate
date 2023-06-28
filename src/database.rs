use std::{error::Error, str::FromStr};
use sqlite::{Connection, ConnectionWithFullMutex, State};
use crate::models::{Language, TranslationLog};

pub fn get_database_connection_manager() -> Result<ConnectionWithFullMutex, Box<dyn Error>> {
    let connection = Connection::open_with_full_mutex("database/slack-translate_production.sqlite")?;

    Ok(connection)
}

pub fn append_to_translation_log(
    user_id: &str,
    user_name: &str,
    language: &Language, 
    message: &str, 
    translation: &str,
    database_connection: &ConnectionWithFullMutex
) -> Result<(), Box<dyn Error>> {
    let insert_query = format!("
        INSERT INTO translation_log (user_id, user_name, language, original_text, translated_text) VALUES (\"{}\", \"{}\", \"{}\", \"{}\",\"{}\")",
        user_id, user_name, language.to_string(), message, translation
    );

    let _ = database_connection.execute(insert_query)?;

    Ok(())
}

pub fn get_translation_log(database_connection: &ConnectionWithFullMutex) -> Result<Vec<TranslationLog>, Box<dyn Error>> {
    let get_translation_log_query = format!("
        SELECT * FROM translation_log
    ");

    let mut statement = database_connection.prepare(get_translation_log_query)?;
    let mut translations = Vec::<TranslationLog>::new();
    while let Ok(State::Row) = statement.next() {
        let id: i64 = statement.read("id")?;
        let user_id: String = statement.read("user_id")?;
        let user_name: String = statement.read("user_name")?;
        let language: String = statement.read("language")?;
        let original_text: String = statement.read("original_text")?;
        let translated_text: String = statement.read("translated_text")?;
        let created: String = statement.read("created")?;

        let translation = TranslationLog::new(
             id as usize,
             user_id,
             user_name,
             Language::from_str(&language).unwrap(),
             original_text,
             translated_text,
             created
        );

        translations.push(translation);

    }

    Ok(translations)
}

