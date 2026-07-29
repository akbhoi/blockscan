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
        let client = Client::builder().user_agent(user_agent).build()?;
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
        // Basic captcha heuristic
        let html_lower = html.to_lowercase();
        if html_lower.contains("cloudflare") && html_lower.contains("challenge")
            || html_lower.contains("verify you are human")
        {
            return Ok((
                Status::Blocked("Captcha/Challenge detected".to_string()),
                None,
                final_url
            ));
        }

        Ok((Status::Allowed, Some(html), final_url))
    }
}
