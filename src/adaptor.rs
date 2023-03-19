use std::collections::HashMap;

use reqwest::{Client, Request, RequestBuilder};
use reqwest_cookie_store::CookieStore;
use serde::Serialize;
use serde_json::{Value, json};


use crate::constants::{INNERTUBE_API_KEY, BASE_URL, INNERTUBE_CLIENT_NAME, INNERTUBE_CLIENT_VERSION};
use crate::innertube_request::{InnertubeRequest, Client as InnertubeClient, Context as InnertubeContext};

const THRESHOLD: i32 = 10;

pub struct Context {
    client: reqwest::Client,
    api_key: String,
	base_url: String,
	client_name: String,
	client_version: String,
	cookie: CookieStore,
	default_fetch_options: Value,
}

impl Context {
    pub fn new() -> Context {
        let mut default_headers = reqwest::header::HeaderMap::new();
    
        Context { 
            client: Client::new(),
            client_name: INNERTUBE_CLIENT_NAME.to_string(),
            client_version: INNERTUBE_CLIENT_VERSION.to_string(),
            api_key: INNERTUBE_API_KEY.to_string(),
            base_url: BASE_URL.to_string(),
            cookie: CookieStore::default(),
            default_fetch_options: json!({})
        }
    } 
}

impl Context {
    pub fn get(&self) {

    }
    
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
            continuation_token: None,
            command_metadata: None,
            browse_endpoint: None,
            playlist_id: None,
            video_id: None,
            browse_id: Some("VLPLaOnVj8qlILfDxxwE_BJ1bgo5lZkL_Xfx".to_string())
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