use std::error::Error;

use crate::models::{DeepLTranslation, SlackPayload, SlackSendMessageRequest};

pub async fn send_translation_reply(translation: &DeepLTranslation, slack_payload: &SlackPayload) -> Result<(), Box<dyn Error>> {
    let url = &slack_payload.response_url;

    let request_body = SlackSendMessageRequest::new(&translation.text, &slack_payload.message_ts);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?; 

    match response.status().is_success() {
        true => Ok(()),
        false => Err("Failed to send message".into())
    }
}
