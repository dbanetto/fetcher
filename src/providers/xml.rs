
use providers::Provider;
use clients::ProviderData;
use url::Url;

pub struct XmlProvider;

impl Provider for XmlProvider {

    fn fetch(&self, prov: ProviderData) -> Result<Vec<(Url,i32)>, String> {
        Err("".to_string())
    }
}
