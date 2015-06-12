use providers::Provider;
use clients::ProviderData;
use url::Url;

pub struct XmlProvider {
    base_url: Url,
    title_xpath: String,
    link_xpath: String,
}

impl XmlProvider {
    fn new(base_url: Url, title_xpath: String, link_xpath: String) -> Self {
        XmlProvider {
            base_url: base_url,
            title_xpath: title_xpath,
            link_xpath: link_xpath,
        }
    }
}

impl Provider for XmlProvider {

    fn fetch(&self, prov: ProviderData) -> Result<Vec<(Url,i32)>, String> {
        Err("".to_string())
    }
}
