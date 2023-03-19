use scraper::Selector;
use serde_json::{Value, Map};
use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;

pub async fn download_text(url: String) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;

    // reqwest::Client::new().get(url).header(key, value);

    // 'x-spf-previous': WATCH_LATER_URL,
    // 'x-spf-referer': WATCH_LATER_URL,
    // 'x-youtube-client-name': str(ytcfg['INNERTUBE_CONTEXT_CLIENT_NAME']),
    // 'x-youtube-client-version': ytcfg['INNERTUBE_CONTEXT_CLIENT_VERSION'],
    // 'x-youtube-page-cl': str(ytcfg['PAGE_CL']),
    // 'x-youtube-utc-offset': '-300',

    Ok(body)
}

pub async fn download_page(url: String) -> Result<scraper::Html, reqwest::Error> {
    let page = download_text(url).await?;
    let document = scraper::Html::parse_document(&page);

    Ok(document)
}

pub async fn save_file(text: &String, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(text.as_bytes())
}

pub fn extract_brace(text: &String, start: usize) -> String {
    let mut end = start;
    let mut unclosed = 0;

    let len = text.len();
    let bytes = text.as_bytes();

    while end < len {
        match bytes[end] {
            b'{' => unclosed = unclosed + 1,
            b'}' => unclosed = unclosed - 1,
            _ => (),
        }

        if unclosed == 0 {
            break;
        }

        end = end + 1;
    }

    end = end + 1;

    String::from_utf8(bytes[start..end].to_vec()).unwrap()
}

pub fn extract_initial_data(text: &String) -> String {
    let search_str = "var ytInitialData = ";
    let start = text.find(search_str).unwrap() + search_str.len();
    let ytInitialData = extract_brace(text, start);

    ytInitialData
}

pub fn extract_ytConfig(text: &String) -> String {
    let search_str = "ytcfg.set({";
    let start = text.find(search_str).unwrap() + search_str.len() - 1;
    let ytConfig = extract_brace(text, start);

    ytConfig
}

pub fn playlist_extractor(playlist: &Vec<Value>) {
    let n = playlist.len();

    for i in 0..n-1 {
        playlist_video_extractor(&playlist[i]["playlistVideoRenderer"]);
    }

    let final_item: &Map<String, Value> = playlist[n-1].as_object().unwrap();
    
    if final_item.contains_key("playlistVideoRenderer") {
        playlist_video_extractor(&final_item["playlistVideoRenderer"]);
    } else {
        playlist_continuation_extractor(&final_item["continuationItemRenderer"])
    }
}

pub fn playlist_video_extractor(playlist_video: &Value) {
    println!("Title: {}", playlist_video);
}

pub fn playlist_continuation_extractor(continuation_item: &Value) {
    let token = continuation_item["continuationEndpoint"]["continuationCommand"]["token"].as_str().unwrap();
    let ctp = continuation_item["continuationEndpoint"]["clickTrackingParams"].as_str().unwrap();
    
    println!("Got continuation token: {}", token);
    println!("got CTP: {}", ctp);


    download_page("https://www.youtube.com/youtubei/v1/browse?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8&prettyPrint=false".to_owned());
}

pub async fn download_playlist_info() {
    let text = download_text(
        "https://www.youtube.com/playlist?list=PLaOnVj8qlILdygCid2vyjkTHkEAZypUyI".to_owned(),
    )
    .await
    .unwrap();
    let _document = scraper::Html::parse_document(&text);

    let st = extract_initial_data(&text);
    save_file(&st, "initialData.json").await.unwrap();
    let st = extract_ytConfig(&text);
    save_file(&st, "ytConfig.json").await.unwrap();

    save_file(&text, "./page.html").await.unwrap();

    let page = download_page(
        "https://www.youtube.com/playlist?list=PLaOnVj8qlILdygCid2vyjkTHkEAZypUyI".to_owned(),
    )
    .await
    .unwrap();

    println!("Done");
}
