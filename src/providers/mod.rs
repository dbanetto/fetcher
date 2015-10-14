
pub mod xml;
pub use self::xml::XmlProvider;

use url::Url;
use clients::ProviderData;

pub struct DummyProvider;

impl Provider for DummyProvider {
    fn fetch(&self, _: ProviderData) -> Result<Vec<(Url,i32)>, String> {
        Err("".to_string())
    }
}

pub trait Provider {
    fn fetch(&self, prov: ProviderData) -> Result<Vec<(Url,i32)>, String>;
}
