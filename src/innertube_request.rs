use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnertubeRequest {
    pub context: Option<Context>,
    pub continuation: Option<String>,
    pub command_metadata: Option<CommandMetadata>,
    pub browse_endpoint: Option<BrowseEndpoint>,
    pub playlist_id: Option<String>,
    pub video_id: Option<String>,
    pub browse_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub client: Option<Client>,
    pub click_tracking: Option<ClickTrackingParams>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClickTrackingParams {
    pub click_tracking_params: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub hl: Option<String>,
    pub gl: Option<String>,
    pub client_name: Option<String>,
    pub client_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandMetadata {
    pub web_command_metadata: Option<WebCommandMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCommandMetadata {
    pub url: Option<String>,
    pub send_post: Option<bool>,
    pub api_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: Option<String>,
}