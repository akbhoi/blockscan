use reqwest::{Client, StatusCode};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Status {
    Allowed,
    Blocked(String), // Reason for being blocked
    Error(String),
}

pub struct Checker {
    client: Client,
}

impl Checker {
    pub fn new(user_agent: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    pub async fn check(&self, url: &str) -> Result<(Status, Option<String>, url::Url), reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let status = response.status();
        let final_url = response.url().clone();

        if status == StatusCode::FORBIDDEN || status == StatusCode::TOO_MANY_REQUESTS {
            return Ok((Status::Blocked(format!("HTTP {}", status)), None, final_url));
        }

        if !status.is_success() {
            return Ok((Status::Error(format!("HTTP {}", status)), None, final_url));
        }

        let html = response.text().await?;
        let detected_status = analyze_html_body(&html);
        if let Status::Blocked(_) = &detected_status {
            return Ok((detected_status, None, final_url));
        }

        Ok((Status::Allowed, Some(html), final_url))
    }
}

pub fn analyze_html_body(html: &str) -> Status {
    let html_lower = html.to_lowercase();
    if (html_lower.contains("cloudflare") && (html_lower.contains("challenge") || html_lower.contains("just a moment")))
        || html_lower.contains("verify you are human")
        || html_lower.contains("g-recaptcha")
        || html_lower.contains("hcaptcha")
        || html_lower.contains("perimeterx")
        || html_lower.contains("datadome")
    {
        Status::Blocked("CAPTCHA / WAF Challenge Detected".to_string())
    } else {
        Status::Allowed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_html_body_allowed() {
        let body = "<html><body><h1>Welcome to Example</h1><a href='/about'>About</a></body></html>";
        assert_eq!(analyze_html_body(body), Status::Allowed);
    }

    #[test]
    fn test_analyze_html_body_cloudflare() {
        let body = "<html><head><title>Just a moment...</title></head><body>Cloudflare challenge page</body></html>";
        assert_eq!(
            analyze_html_body(body),
            Status::Blocked("CAPTCHA / WAF Challenge Detected".to_string())
        );
    }

    #[test]
    fn test_analyze_html_body_recaptcha() {
        let body = "<html><body><p>Please verify you are human to proceed</p><div class='g-recaptcha'></div></body></html>";
        assert_eq!(
            analyze_html_body(body),
            Status::Blocked("CAPTCHA / WAF Challenge Detected".to_string())
        );
    }
}
