use reqwest::blocking::get;
use scraper::{Html, Selector};

fn fetch_url(url: &str) -> String {
    match get(url) {
        Ok(response) => {
            match response.text() {
                Ok(body) => body,
                Err(e) => {
                    eprintln!("Error in retrieving the body: {}", e);
                    return String::new();
                },
            }
        },
        Err(e) => {
            eprintln!("Error in response: {}", e);
            return String::new();
        }
    }
}

fn extract_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = match Selector::parse("a") {
        Ok(selector) => selector,
        Err(e) => {
            eprintln!("Error! {}", e.to_string());
            return Vec::new();
        },
    };
    let mut links = Vec::new();
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(link.to_string());
        }
    }
    links
}

fn main() {
    let url = "http://www.google.com";
    let html_content = fetch_url(url);
    if html_content.is_empty() {
        eprintln!("Failed to fetch content from the URL");
        return;
    }

    println!("Fetched HTML content from: {}", url);
    let links = extract_links(&html_content);
    println!("Extracted Links:");
    for link in links {
        println!("{}", link);
    }
}
