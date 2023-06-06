use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DeepLTranslation {
    pub detected_source_language: String,
    pub text: String
}

#[derive(Debug, Deserialize)]
pub struct DeepLResponse {
    pub translations: Vec<DeepLTranslation>
}