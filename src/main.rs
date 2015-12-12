extern crate hyper;
extern crate rustc_serialize;
extern crate fetcher;
extern crate sxd_document;
extern crate sxd_xpath;

use std::io::Read;

use std::fs::File;

use std::collections::{BTreeMap, HashMap};

use serde_json::Value;

use fetcher::clients::WebClient;
use fetcher::clients::Client;

use fetcher::providers;
use fetcher::providers::Provider;

// fn load_settings() -> Settings {
//     let mut settings_file = File::open("settings.json").unwrap();
//     let mut settings_content = String::new();
//     settings_file.read_to_string(&mut settings_content).unwrap();

//     json::decode::<Settings>(&settings_content).unwrap()
// }

#[cfg(not(test))]
fn main() {
    // let settings = load_settings();
    // let mut base_providers = HashMap::<String, Box<providers::Provider>>::new();

    // base_providers.insert("dummy".to_string(), Box::new(providers::DummyProvider));
    // base_providers.insert("rss".to_string(), Box::new(providers::XmlProvider));

    // // Create a client to fetch Web UI
    // let client = WebClient::new(&settings.fetch_url).unwrap();

    // println!("Base Providers");
    // for base in client.get_base_providers().unwrap() {
    //     if !base_providers.contains_key(&base.name) {
    //         println!("\t{} is an unsupported base provider", base.name);
    //     } else {
    //         println!("\t{}", base.name);
    //     }
    // }

    // println!("\nProviders");
    // for prov in client.get_providers().unwrap() {
    //     println!("\t{} {} ({})",prov.id , prov.name, prov.base_provider);
    // }

    // println!("\nSeries");
    // for series in client.get_series().unwrap() {
    //     println!("\t{}", series.title);
    // }

}
