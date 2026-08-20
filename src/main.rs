use position_and_risk_engine::{create_app, mock_data};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
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

    // Build the Axum application router using the factory
    let app = create_app(state);

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
