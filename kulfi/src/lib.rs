#![deny(unused_extern_crates)]
#![deny(unused_crate_dependencies)]
#![deny(unsafe_code)]

extern crate self as kulfi; // TODO: Remove this and separate kulfi binary from library to get rid of unused_crate_dependencies
// lint check errors
// Only the binary is using the following crates:
use clap as _;
use clap_verbosity_flag as _;
use directories as _;
#[cfg(target_os = "linux")]
use libdbus_sys as _;
use tracing_subscriber as _;

#[cfg(feature = "ui")]
mod browse;
mod config;
pub mod control_server;
mod counters;
mod identity;
pub mod peer_server;
mod start;
#[cfg(feature = "ui")]
mod tauri;
pub mod utils;

pub use config::Config;
pub use counters::{
    CONTROL_CONNECTION_COUNT, CONTROL_REQUEST_COUNT, IN_FLIGHT_REQUESTS,
    OPEN_CONTROL_CONNECTION_COUNT,
};
// pub use identity::{Identity, PeerIdentity};
#[cfg(feature = "ui")]
pub use browse::browse;
pub use identity::Identity;
pub use start::start;
#[cfg(feature = "ui")]
pub use tauri::ui;
