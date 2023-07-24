use std::error::Error;
use crate::models::{DeepLTranslation, SlackPayload, SlackSendMessageRequest, SlackSendErrorRquest};

pub async fn send_translation_reply(translation: &DeepLTranslation, slack_payload: &SlackPayload) -> Result<(), Box<dyn Error>> {
    let url = &slack_payload.response_url;

    let req_body = SlackSendMessageRequest {
        text: translation.text.clone(), 
        thread_ts: slack_payload.message_ts.clone(),
        replace_original: false,
        response_type: String::from("in_channel")
    };

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&req_body)
        .send()
        .await?; 

    match res.status().is_success() {
        true => Ok(()),
        false => Err("Failed to send message".into())
    }
}

pub fn log_error(err: String) {
    tokio::spawn(async {
        let url = "";
        let channel_id = "";
        let req_body = SlackSendErrorRquest {
            channel: channel_id.to_string(),
            text: err 
        };

        let client = reqwest::Client::new();
        client.post(url)
            .json(&req_body)
            .send()
            .await
            .expect("There was a problem logging the error to slack");
    });
}

