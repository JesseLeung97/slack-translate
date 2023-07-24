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

#[derive(Debug, Serialize)]
pub struct SlackSendErrorRquest {
    pub channel: String,
    pub text: String,
}
    
