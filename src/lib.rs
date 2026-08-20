pub mod domain;
pub mod handlers;
pub mod mock_data;
pub mod repository;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub use domain::{
    build_portfolio_response, AssetType, HistoryPoint, PortfolioResponse, PortfolioSummary,
    Position, PositionView,
};
pub use handlers::AppState;
pub use repository::{InMemoryPositionRepository, PositionRepository, RepositoryError};

/// Application factory that instantiates the Axum router with state and middleware.
///
/// Accepts any repository implementing `PositionRepository` trait (Dependency Injection).
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/api/portfolio", get(handlers::get_portfolio))
        .route(
            "/api/positions/:ticker/price",
            post(handlers::update_asset_price),
        )
        .fallback_service(ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
