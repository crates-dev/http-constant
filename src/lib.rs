//! http-constant
//! A comprehensive library providing common HTTP constants for
//! header names, versions, MIME types, and protocol identifiers.

mod common;
mod content_type_value;
mod file_extension;
mod header_key;
mod header_value;
mod http2;
mod http_status;
mod http_version;
mod method;
mod protocol;
mod session;

pub use {
    common::*, content_type_value::*, file_extension::*, header_key::*, header_value::*,
    http_status::*, http_version::*, http2::*, method::*, protocol::*, session::*,
};

use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    time::Duration,
};
