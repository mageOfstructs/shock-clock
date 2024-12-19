use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventPayload;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessibilityEvent {
    pub text: String,
    pub package: String,
    pub event_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GoToHomeScreenArgs;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GoToHomeScreenResult;
