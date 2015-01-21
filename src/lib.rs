#![crate_type = "lib"]
#![crate_name = "suruga"]

#![allow(missing_copy_implementations)]

#![feature(io, core, collections, net)]

#[macro_use]
extern crate log;
extern crate rand;

extern crate chrono;

pub use tls::Tls;
pub use client::TlsClient;

#[macro_use]
pub mod macros;
pub mod util;

#[macro_use]
pub mod der;

// basic crypto primitives
pub mod crypto;

pub mod tls_result;
#[macro_use]
pub mod tls_item;
pub mod record;

// TLS AEAD cipehrsuites
pub mod cipher;

pub mod signature;
pub mod alert;
pub mod handshake;

pub mod tls;
pub mod client;

#[macro_use]
pub mod x509;

#[cfg(test)] mod test;
