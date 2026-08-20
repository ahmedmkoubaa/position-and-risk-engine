pub mod domain;
pub mod handlers;
pub mod mock_data;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub use domain::{
    build_portfolio_response, AssetType, PortfolioResponse, PortfolioSummary, Position,
    PositionView,
};
pub use handlers::AppState;

/// Application factory that instantiates the Axum router with state and middleware.
///
/// Making the application factory public enables seamless in-memory integration testing
/// without binding to physical network ports.
pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/api/portfolio", get(handlers::get_portfolio))
        .fallback_service(ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
