#[allow(clippy::all, warnings)]
pub mod gen;

pub mod builders;
pub mod client;
pub mod transform;
pub mod types;

pub use client::{
    ApiStatusError, BoardTargetError, IpcDocumentObservationError, IpcFailure, KiCadIpcClient,
    TransportUnreachable,
};
pub use types::*;
