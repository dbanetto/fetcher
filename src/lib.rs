//! The `fetcher` create provides a Rust API of the Fetcher Web UI
//!

#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

pub mod clients;
pub mod providers;
