use scraper::{Html, Selector};
use url::Url;

pub fn extract_links(html: &str, base_url: &Url) -> Vec<Url> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(mut url) = base_url.join(href) {
                // Remove fragments to avoid treating same page as different URLs
                url.set_fragment(None);

                // Keep only http/https links
                if url.scheme() == "http" || url.scheme() == "https" {
                    // Check domain match, ignoring "www." prefix
                    let u_domain = url.domain().map(|d| d.strip_prefix("www.").unwrap_or(d));
                    let b_domain = base_url.domain().map(|d| d.strip_prefix("www.").unwrap_or(d));
                    
                    if u_domain.is_some() && u_domain == b_domain {
                        links.push(url);
                    }
                }
            }
        }
    }
    links
}
