mod parse;
mod adaptor;
mod constants;
mod innertube_request;
mod innertube_response;

use std::{fs::read_to_string, hash::Hash};
use std::fmt;

use std::collections::HashMap;
use scraper::{Html, Selector, html};
use serde_json::json;

use innertube_response::{PlaylistItem};

fn hidden_forum_inputs(page: Html) -> HashMap<String, String> {
    let mut inputs: HashMap<String, String> = HashMap::new();

    let mut selector = Selector::parse("input").unwrap();
    let elements = page.select(&selector);

    for element in elements {
        let e = element.value();
        let _type = element.value().attr("type").unwrap_or_default();

        if !_type.eq("hidden") && !_type.eq("submit") {
            continue;
        }

        let identifier = e.attr("name").or(e.id());

        let identifier = match identifier {
            Some(value) => {value},
            None => {continue;}
        };

        let value = e.attr("value").unwrap_or_default();

        inputs.insert(identifier.to_string(), value.to_string());
    }

    inputs
}

fn request(url: String, hidden_inputs: HashMap<String, String>,  f_req: HashMap<String, String>){
    let mut data = hidden_inputs.clone();
    data.insert("pstMsg".to_owned(),"1".to_owned());
    data.insert("checkConnection".to_owned(),"youtube".to_owned());
    data.insert("checkedDomains".to_owned(),"youtube".to_owned());
    data.insert("hl".to_owned(),"en".to_owned());
    data.insert("deviceinfo".to_owned(), "[null,null,null,[],null,\"US\",null,null,[], \"GlifWebSignIn\",null,[null,null,[]]]".to_owned());
    data.insert("f.req".to_owned(), hashmap_to_str(f_req));
    data.insert("flowName".to_owned(),"GlifWebSignIn".to_owned());
    data.insert("flowEntry".to_owned(),"ServiceLogin".to_owned());

    print!("{:?}", data);
}

fn hashmap_to_str(map: HashMap<String, String>) -> String {
    let mut res = String::from("{");
    
    for (k, v) in map {
        res.push_str(format!("\"{}\":\"{}\",", k, v).as_str());
    }

    res.pop();
    res.push_str("}");

    res
}



#[tokio::main]
async fn main() {
    let login_url = "https://accounts.google.com/ServiceLogin".to_owned();
    let lookup_url = "https://accounts.google.com/_/signin/sl/lookup".to_owned();
    let mut client = adaptor::Context::new();

    let endpoint = reqwest::Url::parse("https://www.youtube.com/youtubei/v1/browse").unwrap();

    let mut count = 0;

    let result = client.post(endpoint).await.unwrap();
    let response = result.json::<innertube_response::BrowseResponse>().await.unwrap();
    for tab in response.contents.two_column_browse_results_renderer.tabs {
        if tab.tab_renderer.content.is_none() {
            continue;
        }

        let content = tab.tab_renderer.content.unwrap();
        for section in content.section_list_renderer.contents {
            for item in section.item_section_renderer.contents {
                if item.playlist_video_list_renderer.is_none() {
                    continue;                
                }

                let list = item.playlist_video_list_renderer.unwrap().contents;
                for item in list {
                    match item {
                        PlaylistItem::Video(video) => {
                            count = count + 1;
                            println!("Got video #{}", count);
                        },
                        PlaylistItem::Continuation(continuation) => {
                            println!("Got continuation after {} videos", count);
                        }
                    }

                }
            }
        }


    }
    
    // parse::download_playlist_info().await;
    ()
}
