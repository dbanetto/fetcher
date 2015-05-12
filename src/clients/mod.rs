//! General fetcher data structures and traits
//!
//!

pub mod web_client;
pub use self::web_client::WebClient;

use rustc_serialize::json;
use rustc_serialize::json::Json;

use std::collections::BTreeMap;

///
///
///
pub struct Series {
    pub title: String,
    pub provider_id: u64,
    pub search_title: String,
    pub current_count: u64,
    pub total_count: u64,
    pub media_type: String,
    pub media_type_options: json::Object,
}

///
///
///
pub struct Provider {
    pub id: u64,
    pub name:String,
    pub regex_find_count: String,
    pub base_provider: String,
    pub base_provider_options: json::Object,
}

///
///
///
pub struct BaseProvider {
    pub name: String,
    pub id: u64,
}


///
///
///
pub trait Client { //FIXME: Provide better error handling with a descriptive enum

    ///
    ///
    ///
    fn get_series(&self) -> Result<Vec<self::Series>, String>;

    ///
    ///
    ///
    fn get_providers(&self) -> Result<Vec<self::Provider>, String>;

    ///
    ///
    ///
    fn get_base_providers(&self) -> Result<Vec<self::BaseProvider>, String>;
}

impl BaseProvider { //FIXME: Provide better error handling with a descriptive enum

    /// Parse BaseProvider from a Json Object
    ///
    pub fn parse(json: &Json) -> Result<BaseProvider, String> {
        if let &Json::Object(ref obj) = json {
            Ok(BaseProvider {
                name: match obj.get("name") {
                    Some(val) => {
                        match val {
                            &Json::String(ref s) => s,
                            err @ _ => return Err(format!("name is not a String as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing search_title".to_string()),
                }.to_string(),
                id: match obj.get("id") {
                    Some(val) => {
                        match val {
                            &Json::U64(ref i) => *i,
                            err @ _ => return Err(format!("id is not an integer as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing search_title".to_string()),
                },
            })
        } else {
            Err(format!("Expected Json Object got {:?}", json))
        }
    }
}

impl Provider {

    /// Parse a Provider from a Json Object
    ///
    pub fn parse(json: &Json) -> Result<Provider, String> {
        if let &Json::Object(ref obj) = json {
            Ok(Provider {
                name: match obj.get("name") {
                    Some(val) => {
                        match val {
                            &Json::String(ref s) => s,
                            err @ _ => return Err(format!("name is not a String as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing search_title".to_string()),
                }.to_string(),
                id: match obj.get("id") {
                    Some(val) => {
                        match val {
                            &Json::U64(ref i) => *i,
                            err @ _ => return Err(format!("id is not an integer as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing search_title".to_string()),
                },
                base_provider: match obj.get("base_provider") {
                    Some(val) => {
                        match val {
                            &Json::String(ref i) => i,
                            err @ _ => return Err(format!("base_provider is not an integer as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing base_provider".to_string()),
                }.to_string(),
                regex_find_count: match obj.get("regex_find_count") {
                    Some(val) => {
                        match val {
                            &Json::String(ref i) => i,
                            err @ _ => return Err(format!("regex_find_count is not a string as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing regex_find_count".to_string()),
                }.to_string(),

                base_provider_options: match obj.get("base_provider_options") {
                    Some(val) => {
                        match val {
                            &Json::Object(ref o) => o.clone(),
                            err @ _ => return Err(format!("base_provider is not a Json Object as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing base_provider_options".to_string()),
                },
            })
        } else {
            Err(format!("Expected Json Object got {:?}", json))
        }
    }
}

impl Series {

    /// Parse Series from a Json Object
    ///
    pub fn parse(json: &Json) -> Result<Series, String> {
        // FIXME: refactor repeated match statements
        if let &Json::Object(ref obj) = json {
            Ok(Series {
                title: match obj.get("title") {
                    Some(val) => {
                        match val {
                            &Json::String(ref s) => s,
                            err @ _ => return Err(format!("title is not a String as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing search_title".to_string()),
                }.to_string(),

                search_title: match obj.get("search_title") {
                    Some(val) => {
                        match val {
                            &Json::String(ref s) => s,
                            err @ _ => return Err(format!("seach_title is not a String as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing search_title".to_string()),
                }.to_string(),

                media_type: match obj.get("media_type") {
                    Some(val) => {
                        match val {
                            &Json::String(ref s) => s,
                            err @ _ => return Err(format!("media_type is not a String as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing media_type".to_string()),
                }.to_string(),

                provider_id: match obj.get("provider_id") {
                    Some(val) => {
                        match val {
                            &Json::U64(ref i) => *i,
                            err @ _ => return Err(format!("provider_id is not an integer as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing provider_id".to_string()),
                },

                current_count: match obj.get("current_count") {
                    Some(val) => {
                        match val {
                            &Json::U64(ref i) => *i,
                            err @ _ => return Err(format!("current_count is not an integer as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing current_count".to_string()),
                },

                total_count: match obj.get("total_count") {
                    Some(val) => {
                        match val {
                            &Json::U64(ref i) => *i,
                            err @ _ => return Err(format!("total_count is not an integer as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing total_count".to_string()),
                },

                media_type_options: match obj.get("media_type_options") {
                    Some(val) => {
                        match val {
                            &Json::Object(ref o) => o.clone(),
                            err @ _ => return Err(format!("media_type is not a Json Object as expected but a {:?}", err)),
                        }
                    },
                    _ => return Err("Missing media_type_options".to_string()),
                },
            })
        } else {
            Err(format!("Not a Json Object as expected but {:?}", json))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rustc_serialize::json::Json;

    #[test]
    fn test_series_parse() {
        let s = Series::parse(&Json::from_str(r#"
            {
                "title": "a",
                "search_title": "b",
                "provider_id": 1,
                "media_type": "c",
                "current_count": 2,
                "total_count": 3,
                "media_type_options": {}
            }
        "#).unwrap()).unwrap();

        assert_eq!("a", s.title);
        assert_eq!("b", s.search_title);
        assert_eq!("c", s.media_type);

        assert_eq!(1, s.provider_id);
        assert_eq!(2, s.current_count);
        assert_eq!(3, s.total_count);
    }

    #[test]
    fn test_provider_parse() {
        let p = Provider::parse(&Json::from_str(r#"
            {
                "name": "a",
                "id": 1,
                "base_provider": "b",
                "regex_find_count": "c",
                "base_provider_options": {}
            }
        "#).unwrap()).unwrap();

        assert_eq!("a", p.name);
        assert_eq!("b", p.base_provider);
        assert_eq!("c", p.regex_find_count);

        assert_eq!(1, p.id);
    }

    #[test]
    fn test_base_provider_parse() {
        let b = BaseProvider::parse(&Json::from_str(r#"
            {
                "name": "a",
                "id": 1
            }
        "#).unwrap()).unwrap();

        assert_eq!("a", b.name);
        assert_eq!(1, b.id);
    }
}
