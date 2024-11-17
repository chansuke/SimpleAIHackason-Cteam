use reqwest;
use tokio;

use crate::models::FETCH_URL;

#[tokio::main]
pub async fn scraper(query: &str) -> String {
    let url = format!("{}{}", FETCH_URL, query);
    println!("URL: {}", url);
    match fetch_url_content(&url).await {
        Ok(content) => {
            println!("取得した内容: {}", content);
            return extract_information(&content);
        }
        Err(e) => "エラーが発生しました".to_string(),
    }
}

async fn fetch_url_content(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

fn extract_information(content: &str) -> String {
    if let Some(start) = content.find("<title>") {
        if let Some(end) = content[start..].find("</title>") {
            // Adjusting the end index to ensure extraction works correctly
            let title_start = start + 7; // Skip over the length of "<title>"
            let title_end = start + end;
            return content[title_start..title_end].to_string();
        }
    }
    "情報が見つかりませんでした".to_string()
}
