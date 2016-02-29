extern crate fetcher;

use std::io::prelude::*;
use std::path::Path;

use fetcher::clients::{Client, WebClient};
use fetcher::settings::Settings;

fn main() {
    let settings = match Settings::from_file(Path::new("settings.yml")) {
        Ok(s) => s,
        Err(e) => {
            println!("Error while loading settings: {}", e);
            return;
        }
    };
    let client = WebClient::new(&settings.url).unwrap();

    println!("Base Providers");
    for base in client.get_base_providers().unwrap() {
        println!("\t{}", base.name);
    }

    println!("\nProviders");
    for prov in client.get_providers().unwrap() {
        println!("\t{} {} ({})", prov.id, prov.name, prov.base_provider);
    }

    println!("\nSeries");
    for series in client.get_series().unwrap() {
        println!("\t{}", series.title);
    }
}
