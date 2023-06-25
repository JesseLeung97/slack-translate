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
