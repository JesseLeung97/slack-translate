use std::{ error::Error, collections::HashMap };
use sqlite::ConnectionWithFullMutex;
use crate::database::get_translation_log;
use crate::models::Analytics;

pub fn get_analytics(database_connection: &ConnectionWithFullMutex) -> Result<Analytics, Box<dyn Error>> {
    let translation_log = get_translation_log(&database_connection)?;

    let mut count_by_date = HashMap::<String, usize>::new();
    let mut count_by_team_member = HashMap::<String, usize>::new();
    let mut count_by_language = HashMap::<String, usize>::new();
    let mut total_count: usize = 0;


    for translation in &translation_log {
        count_by_language.entry(translation.language.to_string()).and_modify(|count| *count += 1).or_insert(1);
        let translated_date = translation.created.to_owned().as_str()[..=10].to_string();
        count_by_date.entry(translated_date).and_modify(|count| *count += 1).or_insert(1);
        count_by_team_member.entry(translation.user_name.to_owned()).and_modify(|count| *count += 1).or_insert(1);
        total_count += 1;
    }

    let analytics = Analytics::new( 
        total_count,
        translation_log,
        count_by_date,
        count_by_team_member,
        count_by_language
    );

    Ok(analytics)
}
