pub mod consts;
mod handshake;
pub mod server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
