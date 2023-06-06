use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SlackChannel {
    pub id: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct SlackUser {
    pub id: String,
    pub username: String,
    pub team_id: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct SlackMessage {
    pub client_msg_id: String,
    #[serde(rename="type")]
    pub type_name: String,
    pub text: String,
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct SlackPayload {
    #[serde(rename="type")]
    pub type_name: String,
    pub token: String,
    pub user: SlackUser,
    pub channel: SlackChannel,
    pub response_url: String,
    pub message_ts: String,
    pub message: SlackMessage
}