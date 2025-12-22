//! http-constant
//! A comprehensive library providing common HTTP constants for
//! header names, versions, MIME types, and protocol identifiers.

pub(crate) mod common;
pub(crate) mod content_type_value;
pub(crate) mod file_extension;
pub(crate) mod header_key;
pub(crate) mod header_value;
pub(crate) mod http2;
pub(crate) mod http_status;
pub(crate) mod http_version;
pub(crate) mod method;
pub(crate) mod protocol;
pub(crate) mod session;

pub(crate) use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    time::Duration,
};

pub use common::*;
pub use content_type_value::*;
pub use file_extension::*;
pub use header_key::*;
pub use header_value::*;
pub use http_status::*;
pub use http_version::*;
pub use http2::*;
pub use method::*;
pub use protocol::*;
pub use session::*;
