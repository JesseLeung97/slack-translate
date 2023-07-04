use std::{str::FromStr, sync::Arc};
use axum::Extension;
use axum::{http::StatusCode, Form};
use serde_json::from_str;
use crate::cache::{check_cache, set_cache};
use crate::database::append_to_translation_log;
use crate::deepl::get_translation;
use crate::models::{
    AppState, DeepLPostBody, Language, SlackIncomingTranslationRequest, SlackPayload,
};
use crate::slackbot::send_translation_reply;

pub async fn receive_translation_request(
    state: Extension<Arc<AppState>>,
    Form(req): Form<SlackIncomingTranslationRequest>,
) -> StatusCode {
    
    tokio::spawn(translate(req, state));
    StatusCode::OK
}

async fn translate(req: SlackIncomingTranslationRequest, state: Extension<Arc<AppState>>) -> (StatusCode, String) {
    println!("Received request");

    let form_body = match from_str::<SlackPayload>(req.payload.as_str()) {
        Ok(fb) => fb,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };

    let message_text = &form_body.message.text;
    let user_id = &form_body.user.id;
    let user_name = &form_body.user.username;

    if let Some(cache_check) = check_cache(message_text, state.cache_connection.clone()).await {
        // aA
        // pend to translate log 

        if let Err(_) = append_to_translation_log(
            user_id, 
            user_name,
            &cache_check.source_language,
            &message_text, 
            &cache_check.translated_text, 
            &state.database_connection
        ) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to record translation to the database")
                );
        }
        return (StatusCode::OK, cache_check.translated_text);
    }

    let input_vector = vec![message_text.clone()];

    let deepl_post_body_english = DeepLPostBody::new(&input_vector, Language::EN);
    let deepl_post_body_japanese = DeepLPostBody::new(&input_vector, Language::JA);

    let translated_english = match get_translation(deepl_post_body_english).await {
        Ok(res) => res,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };

    let translated_japanese = match get_translation(deepl_post_body_japanese).await {
        Ok(res) => res,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };

    let translation =
        match Language::from_str(translated_english.detected_source_language.as_str()).unwrap() {
            Language::EN => translated_japanese,
            Language::JA => translated_english,
    };

    if let Err(err) = set_cache(
        &message_text, 
        &translation.text, 
        Language::from_str(translation.detected_source_language.as_str()).unwrap(), 
        state.cache_connection.clone())
        .await {
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()); 
        }

    if let Err(_) = append_to_translation_log(
        user_id, 
        user_name,
        &Language::from_str(&translation.detected_source_language).unwrap(), 
        &message_text, 
        &translation.text, 
        &state.database_connection
    ) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to record translation to the database")
        );
    }

    match send_translation_reply(&translation, &form_body).await {
        Ok(_) => (StatusCode::OK, translation.text),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to translate message"),
        ),
    }
}
