extern crate hyper;
extern crate rustc_serialize;
extern crate fetcher;

use std::io::Read;


use rustc_serialize::json::Json;

use fetcher::client::Client;

fn main() {
    // Create a client to fetch Web UI
    let mut client = Client::new("http://127.0.0.1/").unwrap();

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
