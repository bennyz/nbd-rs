pub mod consts;
mod frame;
pub mod server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
