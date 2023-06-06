use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SlackResponse {
    pub ok: bool,
    pub challenge: Option<String>,
}