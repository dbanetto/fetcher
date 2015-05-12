extern crate hyper;
extern crate rustc_serialize;
extern crate fetcher;

use std::io::Read;

use std::fs::File;

use std::collections::BTreeMap;

use rustc_serialize::json;

use fetcher::clients::WebClient;
use fetcher::clients::Client;


#[derive(RustcDecodable)]
struct Settings {
    // Possible changes:
    //  fetch_save_paths '*', the default, is replaced with
    //  fetch_save_path_default
    //
    //  sort_save_path allows media_type specific paths
    //  and renamed to sort_save_paths, String -> <String,String>
    //
    //  sort_search_path allows for media_type specific paths
    //  [String] -> <String, [String]>
    //

    // fetching settings
    fetch_url: String,
    fetch_save_paths: BTreeMap<String, String>,

    // sorting settings
    sort_save_path: String,
    sort_search_paths: Vec<String>,
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
    let client = WebClient::new(&settings.fetch_url).unwrap();

    println!("Base Providers");
    for base in client.get_base_providers().unwrap() {
        println!("\t{}", base.name);
    }

    println!("\nProviders");
    for prov in client.get_providers().unwrap() {
        println!("\t{} {} ({})",prov.id , prov.name, prov.base_provider);
    }

    println!("\nSeries");
    for series in client.get_series().unwrap() {
        println!("\t{}", series.title);
    }

}
