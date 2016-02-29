
#[cfg(feature = "serde_macros")]
include!("client/mod.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/client.mod.rs"));
