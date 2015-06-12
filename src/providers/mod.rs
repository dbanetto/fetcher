
mod xml;

use url::Url;
use clients::ProviderData;

pub trait Provider {
    fn fetch(&self, prov: ProviderData) -> Result<Vec<(Url,i32)>, String>;
}
