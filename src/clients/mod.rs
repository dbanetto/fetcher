
pub mod web_client;
pub use self::web_client::WebClient;

use rustc_serialize::json;
use rustc_serialize::json::Json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Series {
    pub title: String,
    pub provider_id: i32,
    pub search_title: String,
    pub current_count: i32,
    pub total_count: i32,
    pub media_type: String,
    // media_type_options: Map<String, String>,
}

pub trait Client {
    fn get_series(&mut self) -> Result<Vec<self::Series>, String>;
    fn get_providers(&mut self) -> Result<(), ()>;
    fn get_base_providers(&mut self) -> Result<(), ()>;
}
