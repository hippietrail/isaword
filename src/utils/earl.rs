use reqwest::Client;
use scraper::Html;
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

/// Earl - HTTP client wrapper for web scraping
/// 
/// This is a Rust port of the TypeScript Earl class from:
/// https://github.com/hippietrail/hippiebot.js/blob/main/ute/earl.ts
/// 
/// Wraps reqwest with convenient methods for:
/// - Building URLs with path segments and query parameters
/// - Fetching and parsing JSON
/// - Fetching and parsing HTML DOM
/// - Checking for HTTP redirects
/// 
/// The wrapper makes it easy to compare Rust and TypeScript implementations
/// during debugging - function signatures and behavior are intentionally kept similar.
pub struct Earl {
    url: Url,
    client: Client,
    #[allow(dead_code)]
    headers: Option<HashMap<String, String>>,
}

impl Earl {
    /// Create new Earl instance
    /// origin: e.g. "https://dictionary.cambridge.org"
    /// pathname: e.g. "/dictionary/english/"
    /// params: optional query string parameters
    pub fn new(
        origin: &str,
        pathname: &str,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut url = Url::parse(origin)?;
        url.set_path(pathname);

        if let Some(p) = params {
            for (key, value) in p {
                url.query_pairs_mut().append_pair(key, &value);
            }
        }

        Ok(Earl {
            url,
            client: Client::new(),
            headers: None,
        })
    }

    /// Create Earl with custom headers (e.g., User-Agent)
    pub fn with_headers(
        origin: &str,
        pathname: &str,
        params: Option<HashMap<&str, String>>,
        headers: HashMap<String, String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut url = Url::parse(origin)?;
        url.set_path(pathname);

        if let Some(p) = params {
            for (key, value) in p {
                url.query_pairs_mut().append_pair(key, &value);
            }
        }

        Ok(Earl {
            url,
            client: Client::new(),
            headers: Some(headers),
        })
    }

    pub fn set_basic_pathname(&mut self, pathname: &str) {
        self.url.set_path(pathname);
    }

    pub fn set_pathname(&mut self, pathname: &str) {
        self.url.set_path(pathname);
    }

    pub fn get_pathname(&self) -> &str {
        self.url.path()
    }

    pub fn set_last_path_segment(&mut self, segment: &str) {
        let base = self.url.path();
        // if basic pathname is "/dictionary/english/", add segment to end
        self.url.set_path(&format!("{}{}", base.trim_end_matches('/'), segment));
    }

    pub fn set_search_param(&mut self, key: &str, value: &str) {
        self.url.query_pairs_mut().clear().append_pair(key, value);
    }

    pub fn get_origin(&self) -> String {
        self.url.origin().ascii_serialization()
    }

    pub fn get_url_string(&self) -> String {
        self.url.to_string()
    }

    /// Fetch and parse as JSON
    pub async fn fetch_json(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut req = self.client.get(self.url.as_str());
        
        if let Some(hdrs) = &self.headers {
            for (key, value) in hdrs {
                req = req.header(key, value);
            }
        }
        
        let resp = req.send().await?;
        let json = resp.json().await?;
        Ok(json)
    }

    /// Fetch and parse as HTML DOM
    pub async fn fetch_dom(&self) -> Result<Html, Box<dyn std::error::Error>> {
        let text = self.fetch_text().await?;
        Ok(Html::parse_document(&text))
    }

    /// Fetch raw HTML text
    pub async fn fetch_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut req = self.client.get(self.url.as_str());
        
        if let Some(hdrs) = &self.headers {
            for (key, value) in hdrs {
                req = req.header(key, value);
            }
        }
        
        let resp = req.send().await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// Check if URL redirects (HEAD request)
    pub async fn check_redirect(&self) -> Result<Option<bool>, Box<dyn std::error::Error>> {
        match self
            .client
            .head(self.url.as_str())
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let is_redirect = (300..400).contains(&status);
                Ok(Some(is_redirect))
            }
            Err(e) => {
                // Log but don't fail - matches TS behavior
                eprintln!("[Earl/check_redirect] {}: {}", self.url, e);
                Ok(None)
            }
        }
    }
}
