use std::{error::Error, str::FromStr};
use sqlite::{Connection, ConnectionWithFullMutex, State};
use crate::models::{Language, UserAnalytics, TranslationLog};

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

    let _ = database_connection.execute(insert_query)?;

    Ok(())
}

pub fn get_users(database_connection: &ConnectionWithFullMutex) -> Result<Vec<UserAnalytics>, Box<dyn Error>> {
    let get_users_query = format!("
        SELECT user_id, user_name FROM users
    ");

    let mut statement = database_connection.prepare(get_users_query)?;
    let mut users = Vec::<UserAnalytics>::new();
    while let Ok(State::Row) = statement.next() {
        let user_name: String = statement.read("user_name")?;
        let user_id: String = statement.read("user_id")?;

        let user = UserAnalytics::new(user_id, user_name);

        let _ = users.push(user);
    }

    Ok(users)
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
        let translated_text: String = statement.read("translate_text")?;


        let translation = TranslationLog::new(
             id as usize,
             user_id,
             user_name,
             Language::from_str(&language).unwrap(),
             original_text,
             translated_text
        );

        translations.push(translation);
    }

    Ok(translations)
}