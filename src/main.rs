use position_and_risk_engine::{create_app, InMemoryPositionRepository};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging / tracing subscriber with env filter
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "position_and_risk_engine=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Position & Risk Engine with Repository Layer...");

    // Initialize InMemory repository (can be swapped for PostgresPositionRepository)
    let repository = Arc::new(InMemoryPositionRepository::new());

    // Build the Axum application router with injected repository trait
    let app = create_app(repository);

    // Read port from environment or default to 3000
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Bind to 0.0.0.0 to support containerized network bridging
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Position & Risk Engine running at http://0.0.0.0:{}", port);
    tracing::info!("📊 Dashboard UI: http://localhost:{}/", port);
    tracing::info!("🔗 API Endpoint: http://localhost:{}/api/portfolio", port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
