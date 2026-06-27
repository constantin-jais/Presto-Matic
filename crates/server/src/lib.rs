//! presto-server — the Presto-Matic backend library.
//!
//! The authoritative live-session engine lives here as testable library code;
//! `src/main.rs` is the thin binary entry point. Later slices add the trait
//! seams (SessionStore / Fanout / RateLimiter), the WebSocket handler, and the
//! Biscuit join-link middleware.

pub mod session;
