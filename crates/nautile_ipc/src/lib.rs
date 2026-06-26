//! IPC-ready message protocol for future multi-process Nautile.
pub mod channel;
pub mod endpoint;
pub mod message;
pub mod protocol;
pub mod router;
pub use message::IpcMessage;
