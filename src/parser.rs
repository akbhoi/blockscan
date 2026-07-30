use crate::url_utils::{is_same_domain, normalize_url};
use scraper::{Html, Selector};
use url::Url;

pub fn extract_links(html: &str, base_url: &Url) -> Vec<Url> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(url) = normalize_url(base_url, href) {
                if is_same_domain(base_url, &url) {
                    links.push(url);
                }
            }
        }
    }
    links
}

