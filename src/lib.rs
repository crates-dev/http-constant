pub(crate) mod r#const;

pub use r#const::{
    common::*, content_type_value::*, file_extension::*, header_key::*, header_value::*,
    http_version::*, method::*, protocol::*,
};

pub(crate) use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};
