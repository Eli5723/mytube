use std::collections::HashMap;

use reqwest::Client;
use crate::constants::{INNERTUBE_API_KEY, INNERTUBE_CLIENT_NAME, INNERTUBE_CLIENT_VERSION};
use crate::innertube_request::{InnertubeRequest, Client as InnertubeClient, Context as InnertubeContext};

pub struct Context {
    client: reqwest::Client,
}

impl Context {
    pub fn new() -> Context {
        Context { 
            client: Client::new()
        }
    } 
}

impl Context {
    pub async fn post(& mut self, endpoint: reqwest::Url) -> Result<reqwest::Response, reqwest::Error> {
        let client = InnertubeClient{
            hl: Some("en".to_string()),
            gl: Some("US".to_string()),
            client_name: Some(INNERTUBE_CLIENT_NAME.to_string()),
            client_version: Some(INNERTUBE_CLIENT_VERSION.to_string())
        };

        let context = InnertubeContext{
            client: Some(client),
            click_tracking: None
        };

        let data = InnertubeRequest{
            context: Some(context),
            continuation: None,
            command_metadata: None,
            browse_endpoint: None,
            playlist_id: None,
            video_id: None,
            browse_id: Some("VLPLvC1OTKEr5JMpl165zV1lI_-xUvK9rNRO".to_string())
        };

        let mut params = HashMap::new();
        params.insert("key", INNERTUBE_API_KEY);
        params.insert("prettyPrint", "false");

        self.client.post(endpoint)
        .header("x-youtube-client-version", INNERTUBE_CLIENT_VERSION)
        .header("x-youtube-client-name", INNERTUBE_CLIENT_NAME)
        .header("content-type", "application/json")
        .header("accept-encoding", "gzip, deflate, br")
        .query(&params)
        .json(&data)
        .send()
        .await
    }

    pub async fn post_continuation(& mut self, endpoint: reqwest::Url, contiuation: String) -> Result<reqwest::Response, reqwest::Error> {
        let client = InnertubeClient{
            hl: Some("en".to_string()),
            gl: Some("US".to_string()),
            client_name: Some(INNERTUBE_CLIENT_NAME.to_string()),
            client_version: Some(INNERTUBE_CLIENT_VERSION.to_string())
        };

        let context = InnertubeContext{
            client: Some(client),
            click_tracking: None
        };

        let data = InnertubeRequest{
            context: Some(context),
            continuation: Some(contiuation),
            command_metadata: None,
            browse_endpoint: None,
            playlist_id: None,
            video_id: None,
            browse_id: None
        };

        let mut params = HashMap::new();
        params.insert("key", INNERTUBE_API_KEY);
        params.insert("prettyPrint", "false");

        self.client.post(endpoint)
        .header("x-youtube-client-version", INNERTUBE_CLIENT_VERSION)
        .header("x-youtube-client-name", INNERTUBE_CLIENT_NAME)
        .header("content-type", "application/json")
        .header("accept-encoding", "gzip, deflate, br")
        .query(&params)
        .json(&data)
        .send()
        .await
    }
}