//! A Web client to Fetcher Web UI
//!
//!
use std::io::Read;
use url::{Url, ParseError};

use hyper;
use hyper::client::response::Response;
use hyper::header::ContentType;
use hyper::mime::Mime;
use hyper::header::Connection;
use hyper::header::ConnectionOption;

use serde_json;
use serde_json::Value;

use clients::{Client, SeriesData, ProviderData, BaseProviderData};

///
///
///
pub struct WebClient {
    url: Url,
}

impl WebClient {
    ///
    /// #Example
    ///
    /// ```
    /// use fetcher::clients::WebClient;
    ///
    /// let client = WebClient::new("http://127.0.0.1/").unwrap();
    /// ```
    pub fn new(url: &str) -> Result<WebClient, ParseError> {
        let url_parsed = match Url::parse(url) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        Ok(WebClient {
            url: url_parsed,
        })
    }

    ///
    ///
    ///
    fn build_url(&self, to_add: &str) -> Result<Url, ParseError> {
        let base_url = &self.url.to_string();
        let new_url = if base_url.ends_with('/') ^ to_add.starts_with('/') {
            // One trailing or starting '/'
            format!("{}{}", base_url, to_add)
        } else if base_url.ends_with('/') && to_add.starts_with('/') {
            // Pair of trailing and starting '/'
            format!("{}{}", base_url.trim_right_matches('/'), to_add)
        } else {
            // No trailing or starting '/'
            format!("{}/{}", base_url, to_add)
        };
        Url::parse(&new_url)
    }

    /// Raw access to using the HTTP `GET` method at a given `path`
    /// from the base url given at construction.
    ///
    /// #Example
    ///
    /// ```
    /// use fetcher::clients::Client;
    /// use fetcher::clients::WebClient;
    ///
    /// let client = WebClient::new("http://127.0.0.1/").unwrap();
    ///
    /// client.get("/api/");
    /// ```
    pub fn get(&self, path: &str) -> hyper::error::Result<Response> {
        // FIXME: handle build_url fail case
        // FIXME: Change return type
        let full_url = self.build_url(path).unwrap();
        let client = hyper::Client::new();
        let res = client.get(full_url)
            .header(Connection(vec![ConnectionOption::Close]))
            .header(ContentType("application/json".parse::<Mime>().unwrap()))
            .send();
        res
    }
}

///
///
///
impl Client for WebClient {


    /// Get a list of SeriesData
    ///
    /// # Example
    ///
    /// ```
    /// use fetcher::clients::Client;
    /// use fetcher::clients::WebClient;
    ///
    /// let client = WebClient::new("http://127.0.0.1/").unwrap();
    ///
    /// client.get_series();
    /// ```
    fn get_series(&self) -> Result<Vec<SeriesData>, String> {
        let mut res = match self.get("/series/?format=fetch") {
            Ok(r) => r,
            Err(e) => return Err(format!("Error during GET: {}", e)),
        };

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        match serde_json::de::from_str(&body) {
            Ok(ok) => ok,
            Err(e) => Err(format!("{}", e))
        }
    }

    /// Get a list of Providers
    ///
    /// # Example
    ///
    /// ```
    /// use fetcher::clients::Client;
    /// use fetcher::clients::WebClient;
    ///
    /// let client = WebClient::new("http://127.0.0.1/").unwrap();
    ///
    /// client.get_providers();
    /// ```
    fn get_providers(&self) -> Result<Vec<ProviderData>, String> {
        let mut res = match self.get("/provider/?format=fetch") {
            Ok(r) => r,
            Err(e) => return Err(format!("Error during GET: {}", e)),
        };

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        match serde_json::de::from_str(&body) {
            Ok(ok) => ok,
            Err(e) => Err(format!("{}", e))
        }
    }

    /// Get a list of Base Providers
    ///
    /// # Example
    ///
    /// ```
    /// use fetcher::clients::Client;
    /// use fetcher::clients::WebClient;
    ///
    /// let client = WebClient::new("http://127.0.0.1/").unwrap();
    ///
    /// client.get_base_providers();
    /// ```
    fn get_base_providers(&self) -> Result<Vec<BaseProviderData>, String> {
        let mut res = match self.get("/provider/base/?format=fetch") {
            Ok(r) => r,
            Err(e) => return Err(format!("Error during GET: {}", e)),
        };

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        match serde_json::de::from_str(&body) {
            Ok(ok) => ok,
            Err(e) => Err(format!("{}", e))
        }
    }

}

#[cfg(test)]
mod test{
    extern crate url;

    use url::Url;
    use clients::Client;
    use super::*;


    #[test]
    fn test_url_build() {
        for (url, path, built) in vec![
                ("http://e.co", "/p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co/", "/p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co", "p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co:8080", "p/t/f.html", "http://e.co:8080/p/t/f.html"),
                ("http://e.co", "p/t/f.html?var=1", "http://e.co/p/t/f.html?var=1"),
            ] {
            let c = WebClient::new(url).unwrap();
            let built_url = c.build_url(path);
            assert!(Url::parse(built).unwrap() == built_url.unwrap(),
                    "assertion failed: url:{} path:{} built:{} != expected:{}", url, path, built, Url::parse(built).unwrap());
        }
    }

    #[test]
    fn test_get_series() {
        let client = WebClient::new("http://127.0.0.1:8000/").unwrap();
        // TODO: Check if a valid server exists, if not skip test
        client.get_series();
    }

    #[test]
    fn test_get_providers() {
        let client = WebClient::new("http://127.0.0.1:8000/").unwrap();
        // TODO: Check if a valid server exists, if not skip test
        client.get_providers();
    }

    #[test]
    fn test_get_base_providers() {
        let client = WebClient::new("http://127.0.0.1:8000/").unwrap();
        // TODO: Check if a valid server exists, if not skip test
        client.get_base_providers();
    }
}
