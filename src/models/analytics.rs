use std::collections::HashMap;
use crate::models::Language;

#[derive(Debug)]
pub struct UserAnalytics {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone)]
pub struct TranslationLog {
    pub id: usize,
    pub user_id: String,
    pub user_name: String,
    pub language: Language,
    pub original_text: String,
    pub translated_text: String,
    pub created: String 
}


impl TranslationLog {
    pub fn new(
        id: usize,
        user_id: String,
        user_name: String,
        language: Language,
        original_text: String,
        translated_text: String,
        created: String
    ) -> TranslationLog {
        TranslationLog { 
            id,
            user_id,
            user_name,
            language,
            original_text,
            translated_text,
            created
        }
    }
}


pub struct Analytics {
    pub total_count: usize,
    pub translation_log: Vec<TranslationLog>,
    pub count_by_date: HashMap<String, usize>,
    pub count_by_team_member:  HashMap::<String, usize>,
    pub count_by_language: HashMap::<String, usize>,
}

impl Analytics {
    pub fn new(
        total_count: usize,
        translation_log: Vec<TranslationLog>,
        count_by_date: HashMap<String, usize>,
        count_by_team_member: HashMap<String, usize>,
        count_by_language: HashMap<String, usize>
    ) -> Analytics {
        Analytics {
            total_count,
            translation_log,
            count_by_date, 
            count_by_team_member,
            count_by_language
        }
    }
}

