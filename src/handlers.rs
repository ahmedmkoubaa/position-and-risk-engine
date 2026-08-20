use axum::{extract::State, Json};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{build_portfolio_response, PortfolioResponse, Position};

/// Shared thread-safe application state.
///
/// In a production system, this could interface with an in-memory cache,
/// order management system (OMS), or streaming market data pipeline.
pub type AppState = Arc<RwLock<Vec<Position>>>;

/// HTTP Handler for `GET /api/portfolio`.
///
/// Retrieves all positions from state, executes real-time PnL and risk exposure
/// computations, and returns the serialized JSON payload.
pub async fn get_portfolio(State(state): State<AppState>) -> Json<PortfolioResponse> {
    let positions = state.read().await;
    let response = build_portfolio_response(&positions);
    Json(response)
}
