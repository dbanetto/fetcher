#[cfg(feature = "serde_macros")]
include!("settings.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/settings.rs"));
