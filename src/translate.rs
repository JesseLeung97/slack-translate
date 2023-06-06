use std::str::FromStr;

use axum::{http::StatusCode, Form};
use serde_json::from_str;

use crate::models::{SlackIncomingTranslationRequest, SlackPayload, DeepLPostBody, Language};
use crate::deepl::get_translation;
use crate::slackbot::send_translation_reply;

pub async fn receive_translation_request(Form(req): Form<SlackIncomingTranslationRequest>) -> (StatusCode, String) {
    tokio::spawn(translate(req));
    (StatusCode::OK, String::from("Translation request received"))
}

async fn translate(req: SlackIncomingTranslationRequest) -> (StatusCode, String) {
    println!("Received request");

    let form_body = match from_str::<SlackPayload>(req.payload.as_str()) {
        Ok(fb) => fb,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    };

    let message_text = &form_body.message.text;
    let input_vector = vec![message_text.clone()];

    let deepl_post_body_english = DeepLPostBody::new(&input_vector, Language::EN);
    let deepl_post_body_japanese = DeepLPostBody::new(&input_vector, Language::JA);

    let translated_english = match get_translation(deepl_post_body_english).await {
        Ok(res) => res,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    };

    let translated_japanese = match get_translation(deepl_post_body_japanese).await {
        Ok(res) => res,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    };

    let translation = match Language::from_str(translated_english.detected_source_language.as_str()).unwrap() {
        Language::EN => translated_japanese,
        Language::JA => translated_english,
    };

    match send_translation_reply(&translation, &form_body).await {
        Ok(_) => (StatusCode::OK, translation.text),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Failed to translate message"))
    }
}