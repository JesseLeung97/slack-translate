use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SlackIncomingTranslationRequest {
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub struct SlackSendMessageRequest {
    pub text: String,
    pub response_type: String,
    pub replace_original: bool,
    pub thread_ts: String
}

impl SlackSendMessageRequest {
    pub fn new(text: &str, thread_ts: &str) -> SlackSendMessageRequest {
        SlackSendMessageRequest { 
            text: text.to_string(),
            response_type: String::from("in_channel"),
            replace_original: false,
            thread_ts: thread_ts.to_string()
        }
    }
}