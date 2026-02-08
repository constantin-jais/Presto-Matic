//! presto-server — thin binary entry point. Builds the app and serves it.

use std::net::SocketAddr;

use presto_server::app;
use presto_server::registry::SessionRegistry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Clever Cloud injects `PORT`; default to 8080 for local runs.
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("presto-server listening on {addr}");
    axum::serve(listener, app(SessionRegistry::new())).await?;
    Ok(())
}
