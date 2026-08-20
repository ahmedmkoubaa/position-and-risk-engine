mod domain;
mod handlers;
mod mock_data;

use axum::{routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging / tracing subscriber with env filter
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "position_and_risk_engine=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Position & Risk Engine...");

    // Initialize in-memory state with mock portfolio
    let initial_positions = mock_data::get_mock_positions();
    let state = Arc::new(RwLock::new(initial_positions));

    // Build the Axum application router
    // 1. API endpoint for portfolio analytics: GET /api/portfolio
    // 2. Static file serving (serves index.html at root `/`): fallback to static/
    let app = Router::new()
        .route("/api/portfolio", get(handlers::get_portfolio))
        .fallback_service(ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Bind server to address
    let port = 3000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("🚀 Position & Risk Engine running at http://{}", addr);
    tracing::info!("📊 Dashboard UI: http://{}/", addr);
    tracing::info!("🔗 API Endpoint: http://{}/api/portfolio", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
