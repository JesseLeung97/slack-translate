use std::{ error::Error, collections::HashMap };
use sqlite::ConnectionWithFullMutex;
use crate::database::{ get_translation_log, get_users };
use crate::models::Analytics;

pub fn get_analytics(database_connection: &ConnectionWithFullMutex) -> Result<Analytics, Box<dyn Error>> {
    let translation_log = get_translation_log(&database_connection)?;

    let mut count_by_date = HashMap::<String, usize>::new();
    let mut count_by_team_member = HashMap::<String, usize>::new();
    let mut count_by_language = HashMap::<String, usize>::new();
    let mut total_count: usize = 0;


    for translation in &translation_log {
        count_by_language.entry(translation.language.to_string()).and_modify(|count| *count += 1).or_insert(1);
        count_by_date.entry(translation.created.to_owned()).and_modify(|count| *count += 1).or_insert(1);
        count_by_team_member.entry(translation.user_id.to_owned()).and_modify(|count| *count += 1).or_insert(1);
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

// pub fn get_user_names(database_connection: &ConnectionWithFullMutex) -> Result<HashMap<String, String>, Box<dyn Error>> {
  //  let users = get_users(&database_connection)?;

   // for user in &users {

    

    //}
//}
