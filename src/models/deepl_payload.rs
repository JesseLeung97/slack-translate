use serde::Serialize;
use crate::models::language::Language;

#[derive(Debug, Serialize)]
pub struct DeepLPostBody {
    pub text: Vec<String>,
    pub target_lang: String,
}

impl DeepLPostBody {
    pub fn new(text: &Vec<String>, target_lang: Language) -> DeepLPostBody {
        DeepLPostBody { 
            text: text.to_owned(),
            target_lang: target_lang.to_string()
        }
    }
}