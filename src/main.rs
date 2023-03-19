mod parse;
mod adaptor;
mod constants;
mod innertube_request;
mod innertube_response;
mod util;

use innertube_response::PlaylistItem;

fn parse_browse_results(response: &innertube_response::BrowseResponse) -> Option<String> {
    for tab in response.contents.two_column_browse_results_renderer.tabs.iter() {
        if tab.tab_renderer.content.is_none() {
            continue;
        }

        let content = tab.tab_renderer.content.as_ref().unwrap();
        for section in content.section_list_renderer.contents.iter() {
            for item in section.item_section_renderer.contents.iter() {
                if item.playlist_video_list_renderer.is_none() {
                    continue;                
                }

                let list: &Vec<PlaylistItem> = item.playlist_video_list_renderer.as_ref().unwrap().contents.as_ref();
                for item in list {
                    match item {
                        PlaylistItem::Video(video) => {
                            let title = video.playlist_video_renderer.title.runs.first().unwrap().text.clone();
                        },
                        PlaylistItem::Continuation(continuation) => {
                            let token = &continuation.continuation_item_renderer.continuation_endpoint.continuation_command.token;
                            return Some(token.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

fn parse_continuation_items(response: &innertube_response::BrowseResponse) -> Option<String> {
    if let Some(on_response_received_actions) = response.on_response_received_actions.as_ref() {
        for action in on_response_received_actions {
            let list: &Vec<PlaylistItem> = action.append_continuation_items_action.continuation_items.as_ref();
            for item in list {
                match item {
                    PlaylistItem::Video(video) => {
                        let title = video.playlist_video_renderer.title.runs.first().unwrap().text.clone();
                        println!("Title: {}", title);
                    },
                    PlaylistItem::Continuation(continuation) => {
                        println!("Got continuation!");
                        let token = &continuation.continuation_item_renderer.continuation_endpoint.continuation_command.token;
                        return Some(token.to_string());
                    }
                }
            }
        }
    }

    None
}


#[tokio::main]
async fn main() {
    let mut client = adaptor::Context::new();

    let endpoint = reqwest::Url::parse("https://www.youtube.com/youtubei/v1/browse").unwrap();

    let result = client.post(endpoint).await.unwrap();
    let response = result.json::<innertube_response::BrowseResponse>().await.unwrap();
    
    let mut continuation = parse_browse_results(&response);
    while let Some(token) = continuation {
        let endpoint = reqwest::Url::parse("https://www.youtube.com/youtubei/v1/browse").unwrap();
        let continuation_response = client.post_continuation(endpoint, token)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        continuation = parse_continuation_items(&continuation_response);
    }


    ()
}
