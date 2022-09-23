use scraper::{Html, Selector};
use bytes::Bytes;

pub async fn download_text(url: String) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

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

pub async fn download_playlist_info(){
    let text = download_text("https://www.youtube.com/playlist?list=PLaOnVj8qlILfSqRsNmTTiDeK_P_u3THUu".to_owned()).await.unwrap();
    println!("{}", text);

    let page = download_page("https://www.youtube.com/playlist?list=PLaOnVj8qlILfSqRsNmTTiDeK_P_u3THUu".to_owned()).await.unwrap();

    let thumbnailSelector = Selector::parse("#thumbnail").unwrap();

    let thumbnails = page.select(&thumbnailSelector);
    for element in thumbnails {
        let link = element.value().attr("href").unwrap_or_default();
        println!("Link: {}", link);
    }

    println!("Done");
}