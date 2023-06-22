use crate::models::Language;

#[derive(Debug)]
pub struct UserAnalytics {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug)]
pub struct TranslationLog {
    id: usize,
    user_id: String,
    user_name: String,
    language: Language,
    original_text: String,
    translated_text: String
}

pub enum Analytics {
    Total(usize),
    En(usize),
    Ja(usize),
    User(UserAnalytics)
}

impl UserAnalytics {
    pub fn new(user_id: String, user_name: String) -> UserAnalytics {
        UserAnalytics { 
            user_id, 
            user_name 
        }
    }
}

impl TranslationLog {
    pub fn new(
        id: usize,
        user_id: String,
        user_name: String,
        language: Language,
        original_text: String,
        translated_text: String
    ) -> TranslationLog {
        TranslationLog { 
            id,
            user_id,
            user_name,
            language,
            original_text,
            translated_text
        }
    }
}