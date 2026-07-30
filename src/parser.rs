use crate::url_utils::{is_same_domain, normalize_url};
use scraper::{Html, Selector};
use url::Url;

pub fn extract_links(html: &str, base_url: &Url) -> Vec<Url> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        let valid_url = element
            .value()
            .attr("href")
            .and_then(|href| normalize_url(base_url, href))
            .filter(|url| is_same_domain(base_url, url));

        if let Some(url) = valid_url {
            links.push(url);
        }
    }
    links
}
