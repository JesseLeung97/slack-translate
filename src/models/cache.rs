use crate::models::Language;


pub struct CacheHit {
    pub source_language: Language,
    pub translated_text: String,
}

impl CacheHit {
    pub fn new(source_language: Language, translated_text: String) -> CacheHit {
        CacheHit {
            source_language,
            translated_text
        }
    }
}

