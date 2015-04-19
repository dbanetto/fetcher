
// use self::core::str::FromStr;
use url::{Url, ParseError};

use hyper;
use hyper::client::response::Response;
use hyper::header::ContentType;
use hyper::mime::Mime;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::error::HttpResult;


///
///
///
pub struct Client {
    url: Url,
    client: hyper::Client,
}

///
///
///
impl Client {

    ///
    ///
    ///
    pub fn new(url: &str) -> Result<Client, ParseError> {
        let url_parsed = match Url::parse(url) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        Ok(Client {
            url: url_parsed,
            client: hyper::Client::new(),
        })
    }

    ///
    ///
    ///
    fn build_url(&self, to_add: &str) -> Result<Url, ParseError> {
        let base_url = &self.url.to_string();
        let new_url: String;
        if base_url.ends_with('/') ^ to_add.starts_with('/') {
            // One trailing or starting '/'
            new_url = format!("{}{}", base_url, to_add).to_string();
        } else if base_url.ends_with('/') && to_add.starts_with('/') {
            // Pair of trailing and starting '/'
            new_url = format!("{}{}", base_url.trim_right_matches('/'), to_add).to_string();
        } else {
            // No trailing or starting '/'
            new_url = format!("{}/{}", base_url, to_add).to_string();
        }

        Url::parse(&new_url)
    }

    ///
    ///
    ///
    pub fn get(&mut self, path: &str) -> HttpResult<Response> {
        let full_url = self.build_url(path).unwrap();
        self.client.get(full_url)
            // set a header
            .header(Connection(vec![ConnectionOption::Close]))
            .header(ContentType("application/json".parse::<Mime>().unwrap()))
            // let 'er go!
            .send()
    }

    ///
    ///
    ///
    pub fn get_series(&mut self) -> Result<(), ()> {
        unimplemented!();
    }

    ///
    ///
    ///
    pub fn get_providers(&mut self) -> Result<(), ()> {
        unimplemented!();
    }

    ///
    ///
    ///
    pub fn get_base_providers(&mut self) -> Result<(), ()> {
        unimplemented!();
    }

}

#[cfg(test)]
mod test{
    extern crate url;
    use url::Url;
    use super::*;


    #[test]
    fn test_url_build() {
        for (url, path, built) in vec![
                ("http://e.co", "/p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co/", "/p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co", "p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co", "p/t/f.html?var=1", "http://e.co/p/t/f.html?var=1"),
            ] {
            let c = Client::new(url).unwrap();
            let built_url = c.build_url(path);
            assert!(Url::parse(built).unwrap() == built_url.unwrap(),
                    "assertion failed: url:{} path:{} built:{} expected:{}", url, path, built, Url::parse(built).unwrap());
        }
    }
}
