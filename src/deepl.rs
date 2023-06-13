use dotenv::dotenv;
use serde_json::from_str;
use std::{env, error::Error};

use crate::models::{DeepLPostBody, DeepLResponse, DeepLTranslation};

pub async fn get_translation(post_body: DeepLPostBody) -> Result<DeepLTranslation, Box<dyn Error>> {
    dotenv().ok();

    let url = "https://api-free.deepl.com/v2/translate";
    let deepl_auth_token = env::var("DEEPL_AUTH_TOKEN").expect("DEEPL_AUTH_TOKEN must be defined");

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("DeepL-Auth-Key {}", deepl_auth_token),
        )
        .json(&post_body)
        .send()
        .await?;

    let deepl_response = from_str::<DeepLResponse>(response.text().await?.as_str())?;

    if let Some(translation) = deepl_response.translations.first() {
        return Ok(translation.to_owned());
    } else {
        return Err("Translation failed".into());
    }
}
