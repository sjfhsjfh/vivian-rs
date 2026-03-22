// Copyright 2025 hanasaki <hanasakayui2022@gmail.com>. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(all(
    feature = "default-tls",
    any(
        feature = "rustls-tls",
        feature = "rustls-tls-native-roots",
        feature = "rustls-tls-webpki-roots",
    ),
))]
compile_error!(
    "TLS backend features are mutually exclusive. Use default `default-tls`, or disable default features and enable one of: `rustls-tls`, `rustls-tls-native-roots`, `rustls-tls-webpki-roots`."
);

#[cfg(all(
    feature = "rustls-tls-native-roots",
    any(feature = "rustls-tls", feature = "rustls-tls-webpki-roots"),
))]
compile_error!(
    "Enable only one rustls roots option: use `rustls-tls`/`rustls-tls-webpki-roots`, or `rustls-tls-native-roots`."
);

#[cfg(not(any(
    feature = "default-tls",
    feature = "rustls-tls",
    feature = "rustls-tls-native-roots",
    feature = "rustls-tls-webpki-roots",
)))]
compile_error!(
    "A TLS backend feature must be enabled. Enable default `default-tls` (default), or disable default features and enable one rustls feature."
);

pub mod api;
pub mod client;
pub mod error;
pub mod logger;
pub mod types;
pub mod utils;

pub use client::MilkyClient;
pub use error::{MilkyError, Result};
pub use types::communication::{Communication, WebHookConfig, WebSocketConfig};

pub mod prelude {
    pub use milky_types::common::*;
    pub use milky_types::{Event, EventKind, MessageEvent};
    pub use milky_types::friend::*;
    pub use milky_types::group::*;
    pub use milky_types::message::in_coming::*;
    pub use milky_types::message::out_going::*;
}
