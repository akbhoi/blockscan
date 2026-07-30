use crate::checker::{Checker, Status};
use crate::parser::extract_links;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::{Mutex, Semaphore};
use url::Url;

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CrawlResult {
    pub url: String,
    pub parent_url: Option<String>,
    pub status: Status,
    pub depth: usize,
}

pub struct Crawler {
    checker: Arc<Checker>,
    max_depth: usize,
    semaphore: Arc<Semaphore>,
    visited: Arc<Mutex<HashMap<String, CrawlResult>>>,
    pub completed_count: Arc<AtomicUsize>,
    pub verbose: bool,
}

impl Crawler {
    pub fn new(checker: Checker, max_depth: usize, concurrency: usize, verbose: bool) -> Self {
        Self {
            checker: Arc::new(checker),
            max_depth,
            semaphore: Arc::new(Semaphore::new(concurrency)),
            visited: Arc::new(Mutex::new(HashMap::new())),
            completed_count: Arc::new(AtomicUsize::new(0)),
            verbose,
        }
    }

    pub async fn run(&self, start_url: Url) -> Vec<CrawlResult> {
        let mut queue = vec![(start_url, None, 0)];

        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            let mut handles = Vec::new();

            for (url, parent_url, depth) in queue {
                let url_str = url.to_string();

                let mut visited = self.visited.lock().await;
                if visited.contains_key(&url_str) {
                    continue;
                }
                // Mark as visited tentatively
                visited.insert(
                    url_str.clone(),
                    CrawlResult {
                        url: url_str.clone(),
                        parent_url: parent_url.clone(),
                        status: Status::Error("Pending".to_string()),
                        depth,
                    },
                );
                drop(visited);

                let checker = self.checker.clone();
                let semaphore = self.semaphore.clone();
                let visited_map = self.visited.clone();
                let completed_count_clone = self.completed_count.clone();
                let verbose = self.verbose;

                let handle = tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    if verbose {
                        eprintln!("[VERBOSE] [Depth {}] Fetching URL: {}", depth, url_str);
                    }

                    let check_res = checker.check(&url_str).await;

                    let (status, new_links) = match check_res {
                        Ok((s, html_opt, final_url)) => {
                            let extracted = match (&s, html_opt) {
                                (Status::Allowed, Some(html)) => extract_links(&html, &final_url),
                                _ => Vec::new(),
                            };
                            (s, extracted)
                        }
                        Err(e) => (Status::Error(e.to_string()), Vec::new()),
                    };

                    if verbose {
                        eprintln!(
                            "[VERBOSE] [Depth {}] Response for {}: status={:?}, links_found={}",
                            depth,
                            url_str,
                            status,
                            new_links.len()
                        );
                    }

                    let mut visited = visited_map.lock().await;
                    if let Some(res) = visited.get_mut(&url_str) {
                        res.status = status;
                    }
                    drop(visited);

                    completed_count_clone.fetch_add(1, Ordering::Relaxed);

                    (url_str, depth, new_links)
                });
                handles.push(handle);
            }

            for handle in handles {
                if let Ok((parent_url_str, current_depth, new_links)) = handle.await {
                    let within_depth_limit = self.max_depth == 0 || current_depth < self.max_depth;
                    if within_depth_limit {
                        for link in new_links {
                            next_queue.push((
                                link,
                                Some(parent_url_str.clone()),
                                current_depth + 1,
                            ));
                        }
                    }
                }
            }
            queue = next_queue;
        }

        let visited = self.visited.lock().await;
        let mut results: Vec<CrawlResult> = visited.values().cloned().collect();
        results.sort_by(|a, b| {
            a.depth
                .cmp(&b.depth)
                .then_with(|| {
                    a.parent_url
                        .as_deref()
                        .unwrap_or("-")
                        .cmp(b.parent_url.as_deref().unwrap_or("-"))
                })
                .then_with(|| a.url.cmp(&b.url))
        });
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crawler_initialization() {
        let checker = Checker::new("test-agent/1.0").unwrap();
        let crawler = Crawler::new(checker, 2, 5, false);
        assert_eq!(crawler.max_depth, 2);
        assert!(!crawler.verbose);
        assert_eq!(crawler.completed_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_crawl_result_sorting() {
        let mut results = vec![
            CrawlResult {
                url: "https://example.com/b".to_string(),
                parent_url: Some("https://example.com".to_string()),
                status: Status::Allowed,
                depth: 1,
            },
            CrawlResult {
                url: "https://example.com/a".to_string(),
                parent_url: Some("https://example.com".to_string()),
                status: Status::Allowed,
                depth: 1,
            },
            CrawlResult {
                url: "https://example.com".to_string(),
                parent_url: None,
                status: Status::Allowed,
                depth: 0,
            },
        ];

        results.sort_by(|a, b| {
            a.depth
                .cmp(&b.depth)
                .then_with(|| {
                    a.parent_url
                        .as_deref()
                        .unwrap_or("-")
                        .cmp(b.parent_url.as_deref().unwrap_or("-"))
                })
                .then_with(|| a.url.cmp(&b.url))
        });

        assert_eq!(results[0].url, "https://example.com");
        assert_eq!(results[1].url, "https://example.com/a");
        assert_eq!(results[2].url, "https://example.com/b");
    }
}
