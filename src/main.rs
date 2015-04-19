extern crate hyper;
extern crate rustc_serialize;
extern crate fetcher;

use std::io::Read;
use std::fs::File;

use rustc_serialize::json;
use rustc_serialize::json::Json;

use fetcher::client::Client;

#[derive(RustcDecodable)]
struct Settings {
    fetch_url: String,
}

fn load_settings() -> Settings {
    let mut settings_file = File::open("settings.json").unwrap();
    let mut settings_content = String::new();
    settings_file.read_to_string(&mut settings_content).unwrap();

    json::decode::<Settings>(&settings_content).unwrap()
}

#[cfg(not(test))]
fn main() {
    let settings = load_settings();

    // Create a client to fetch Web UI
    let mut client = Client::new(&settings.fetch_url).unwrap();

    // get '/series/'
    let mut res = client.get("series").unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();


    if res.status.is_success() {
        match Json::from_str(&body).unwrap() {
            Json::Array(arr) => {
                for i in arr {
                    println!("Object: {}\n", i);
                }
            },
            _ => {},
        }
    } else {
        println!("Response: {}", body);
    }
    println!("Status: {0}", res.status);
}
