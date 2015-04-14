
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
    fn build_url(&self, to_add: &str) -> Url {
        let mut build_url = self.url.clone();
        {
            let build_url_path = build_url.path_mut().unwrap();
            if build_url_path.len() >= 1 &&
                build_url_path[build_url_path.len() - 1] == "" {
                    build_url_path.pop();
            }

            for part in to_add.split("/") {
                if part != "" {
                    build_url_path.push(part.to_string());
                }
            }
        }
        return build_url;
    }

    ///
    ///
    ///
    pub fn get(&mut self, path: &str) -> HttpResult<Response> {
        let full_url = self.build_url(path);
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
    pub fn get_series() -> Result<(), ()> {
        unimplemented!();
    }

    ///
    ///
    ///
    pub fn get_providers() -> Result<(), ()> {
        unimplemented!();
    }

    ///
    ///
    ///
    pub fn get_base_providers() -> Result<(), ()> {
        unimplemented!();
    }

}

#[cfg(test)]
mod test{
    extern crate url;
    use super::*;


    #[test]
    fn test_url_build() {
        for (url, path, built) in vec![
                ("http://e.co", "/p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co/", "/p/t/f.html", "http://e.co/p/t/f.html"),
                ("http://e.co", "p/t/f.html", "http://e.co/p/t/f.html"),
            ] {
            let c = Client::new(url).unwrap();
            let built_url = c.build_url(path);
            assert!(Url::parse(built).unwrap() == built_url,
                    "assertion failed: url:{} path:{} built:{}", url, path, built);
        }
    }
}
