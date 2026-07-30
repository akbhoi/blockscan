use url::Url;

/// Normalizes a URL by stripping fragment identifier (`#...`) and returning normalized Url object.
pub fn normalize_url(base: &Url, relative_or_absolute: &str) -> Option<Url> {
    let mut parsed = base.join(relative_or_absolute).ok()?;
    parsed.set_fragment(None);
    if parsed.scheme() == "http" || parsed.scheme() == "https" {
        Some(parsed)
    } else {
        None
    }
}

/// Checks if target_url belongs strictly to the exact same top-level domain as base_url (ignoring `www.` prefix).
pub fn is_same_domain(base_url: &Url, target_url: &Url) -> bool {
    let base_domain = base_url.domain().map(|d| d.strip_prefix("www.").unwrap_or(d));
    let target_domain = target_url.domain().map(|d| d.strip_prefix("www.").unwrap_or(d));

    match (base_domain, target_domain) {
        (Some(b), Some(t)) => b == t,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url_relative_paths() {
        let base = Url::parse("https://example.com/docs/index.html").unwrap();
        let normalized = normalize_url(&base, "page2.html").unwrap();
        assert_eq!(normalized.as_str(), "https://example.com/docs/page2.html");

        let normalized_abs = normalize_url(&base, "/about").unwrap();
        assert_eq!(normalized_abs.as_str(), "https://example.com/about");
    }

    #[test]
    fn test_normalize_url_strips_fragments() {
        let base = Url::parse("https://example.com").unwrap();
        let normalized = normalize_url(&base, "/features#header").unwrap();
        assert_eq!(normalized.as_str(), "https://example.com/features");
    }

    #[test]
    fn test_normalize_url_rejects_non_http() {
        let base = Url::parse("https://example.com").unwrap();
        assert!(normalize_url(&base, "mailto:test@example.com").is_none());
        assert!(normalize_url(&base, "javascript:void(0)").is_none());
    }

    #[test]
    fn test_is_same_domain() {
        let base = Url::parse("https://example.com").unwrap();
        let same_www = Url::parse("https://www.example.com/sub/path").unwrap();
        let diff_domain = Url::parse("https://google.com").unwrap();
        let diff_subdomain = Url::parse("https://api.example.com").unwrap();

        assert!(is_same_domain(&base, &same_www));
        assert!(!is_same_domain(&base, &diff_domain));
        assert!(!is_same_domain(&base, &diff_subdomain));
    }
}
