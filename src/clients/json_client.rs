use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use serde_json;

use clients::{Client, SeriesData, ProviderData, BaseProviderData};

/// A `Client` that is backed by data from JSON files.
/// > note: The files are read every time a `Client` function is called
pub struct JsonClient<'a> {
    series_path: &'a Path,
    providers_path: &'a Path,
    base_providers_path: &'a Path,
}

impl<'a> JsonClient<'a> {
    /// Create `JsonClient` with paths pointing directly to each 
    /// file used for backing data.
    ///
    pub fn new(series_path: &'a str,
               providers_path: &'a str,
               base_providers_path: &'a str)
               -> JsonClient<'a> {
        JsonClient {
            series_path: Path::new(series_path),
            providers_path: Path::new(providers_path),
            base_providers_path: Path::new(base_providers_path),
        }
    }
}

impl<'a> Client for JsonClient<'a> {
    ///
    ///
    ///
    fn get_series(&self) -> Result<Vec<SeriesData>, String> {
        let mut file = match File::open(self.series_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Error while opening file: {}", e)),
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(format!("Error while reading file: {}", e));
        }

        match serde_json::from_str(&contents) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(format!("{}", e)),
        }
    }

    ///
    ///
    ///
    fn get_providers(&self) -> Result<Vec<ProviderData>, String> {
        let mut file = match File::open(self.providers_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Error while opening file: {}", e)),
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(format!("Error while reading file: {}", e));
        }

        match serde_json::from_str(&contents) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(format!("{}", e)),
        }
    }

    ///
    ///
    ///
    fn get_base_providers(&self) -> Result<Vec<BaseProviderData>, String> {
        let mut file = match File::open(self.base_providers_path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Error while opening file: {}", e)),
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(format!("Error while reading file: {}", e));
        }

        match serde_json::from_str(&contents) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clients::{Client, SeriesData, ProviderData, BaseProviderData};

    #[test]
    fn test_create_JsonClient() {
        let _ = JsonClient::new("series", "prov", "base_prov");
    }

    #[test]
    fn test_fail_to_open_nonexisting_files() {
        let client = JsonClient::new("series", "prov", "base_prov");

        match client.get_series() {
            Ok(_) => panic!(),
            Err(_) => (),
        }

        match client.get_providers() {
            Ok(_) => panic!(),
            Err(_) => (),
        }

        match client.get_base_providers() {
            Ok(_) => panic!(),
            Err(_) => (),
        }

    }

    #[test]
    fn test_open_existing_files() {
        let client = JsonClient::new("tests/series.json",
                                     "tests/provider.json",
                                     "tests/base_provider.json");

        match client.get_series() {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }

        match client.get_providers() {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }

        match client.get_base_providers() {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }

    }
}
