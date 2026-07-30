use crate::checker::{Checker, Status};
use crate::parser::extract_links;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use url::Url;

#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CrawlResult {
    pub url: String,
    pub status: Status,
    pub depth: usize,
}

pub struct Crawler {
    checker: Arc<Checker>,
    max_depth: usize,
    semaphore: Arc<Semaphore>,
    visited: Arc<Mutex<HashMap<String, CrawlResult>>>,
    pub completed_count: Arc<AtomicUsize>,
}

impl Crawler {
    pub fn new(checker: Checker, max_depth: usize, concurrency: usize) -> Self {
        Self {
            checker: Arc::new(checker),
            max_depth,
            semaphore: Arc::new(Semaphore::new(concurrency)),
            visited: Arc::new(Mutex::new(HashMap::new())),
            completed_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub async fn run(&self, start_url: Url) -> Vec<CrawlResult> {
        let mut queue = vec![(start_url, 0)];

        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            let mut handles = Vec::new();

            for (url, depth) in queue {
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
                        status: Status::Error("Pending".to_string()),
                        depth,
                    },
                );
                drop(visited);

                let checker = self.checker.clone();
                let semaphore = self.semaphore.clone();
                let visited_map = self.visited.clone();
                let completed_count_clone = self.completed_count.clone();

                let handle = tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    let check_res = checker.check(&url_str).await;

                    let (status, new_links) = match check_res {
                        Ok((s, html_opt, final_url)) => {
                            let mut extracted = Vec::new();
                            if s == Status::Allowed {
                                if let Some(html) = html_opt {
                                    extracted = extract_links(&html, &final_url);
                                }
                            }
                            (s, extracted)
                        }
                        Err(e) => (Status::Error(e.to_string()), Vec::new()),
                    };

                    let mut visited = visited_map.lock().await;
                    if let Some(res) = visited.get_mut(&url_str) {
                        res.status = status;
                    }
                    drop(visited);

                    completed_count_clone.fetch_add(1, Ordering::Relaxed);

                    (depth, new_links)
                });
                handles.push(handle);
            }

            for handle in handles {
                if let Ok((current_depth, new_links)) = handle.await {
                    if self.max_depth == 0 || current_depth < self.max_depth {
                        for link in new_links {
                            next_queue.push((link, current_depth + 1));
                        }
                    }
                }
            }
            queue = next_queue;
        }

        let visited = self.visited.lock().await;
        visited
            .values()
            .map(|v| CrawlResult {
                url: v.url.clone(),
                status: v.status.clone(),
                depth: v.depth,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crawler_initialization() {
        let checker = Checker::new("test-agent/1.0").unwrap();
        let crawler = Crawler::new(checker, 2, 5);
        assert_eq!(crawler.max_depth, 2);
        assert_eq!(crawler.completed_count.load(Ordering::Relaxed), 0);
    }
}
