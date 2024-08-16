use std::net::SocketAddr;

pub mod serve;

pub struct QuickServer {
    host: SocketAddr,
    root: String,
    index: String,
}

impl QuickServer {
    pub fn new(
        host: impl Into<SocketAddr>,
        root: impl Into<String>,
        index: impl Into<String>,
    ) -> Self {
        Self {
            host: host.into(),
            root: root.into(),
            index: index.into(),
        }
    }
}
