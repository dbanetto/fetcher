
pub mod web_client;
pub use self::web_client::WebClient;

///
///
///
#[derive(RustcDecodable, RustcEncodable)]
pub struct Series {
    pub title: String,
    pub provider_id: i32,
    pub search_title: String,
    pub current_count: i32,
    pub total_count: i32,
    pub media_type: String,
    // pub media_type_options: Json::Object, //FIXME: not decodable
}

///
///
///
#[derive(RustcDecodable, RustcEncodable)]
pub struct Provider {
    pub id: i32,
    pub name:String,
    pub regex_find_count: String,
    pub base_provider: i32,
    // pub base_provider_options: Json::Object, //FIXME: not decodable
}

///
///
///
#[derive(RustcDecodable, RustcEncodable)]
pub struct BaseProvider {
    pub name: String,
    pub id: i32,
}

///
///
///
pub trait Client {

    ///
    ///
    ///
    fn get_series(&self) -> Result<Vec<self::Series>, String>;

    ///
    ///
    ///
    fn get_providers(&self) -> Result<Vec<self::Provider>, ()>;

    ///
    ///
    ///
    fn get_base_providers(&self) -> Result<Vec<self::BaseProvider>, ()>;
}
