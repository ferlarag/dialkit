//! Shared HTTP transport, authentication, resilience, and pagination for dialkit.

pub mod auth;
pub mod error;
pub mod pagination;
pub mod request;
pub mod response;
pub mod retry;
pub mod trace;
