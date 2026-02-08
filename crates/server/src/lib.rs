//! presto-server — the Presto-Matic backend library.
//!
//! The authoritative live-session engine ([`session`]), the in-memory registry +
//! fanout ([`registry`]), and the WebSocket handler ([`ws`]) live here as testable
//! library code; `src/main.rs` is the thin binary entry point. Later slices add
//! the Biscuit join-link middleware and the distributed (Redis/Postgres) seams.

pub mod registry;
pub mod session;
pub mod ws;

use axum::{Router, routing::get};

use registry::SessionRegistry;

/// Build the application router over a shared session registry.
pub fn app(registry: SessionRegistry) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ws/{session_id}", get(ws::ws_handler))
        .with_state(registry)
}

async fn health() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_returns_ok() {
        let response = app(SessionRegistry::new())
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
