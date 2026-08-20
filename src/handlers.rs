use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::domain::{build_portfolio_response, PortfolioResponse};
use crate::repository::PositionRepository;

/// Shared application state parameterized by the `PositionRepository` trait.
///
/// Enables dependency injection, allowing easy substitution with real database adapters
/// (e.g. PostgreSQL, Redis) or mock test doubles.
pub type AppState = Arc<dyn PositionRepository>;

/// HTTP Handler for `GET /api/portfolio`.
///
/// Fetches positions from the repository trait, calculates real-time Mark-to-Market PnL,
/// exposure and risk allocation, and returns the response DTO.
pub async fn get_portfolio(
    State(repo): State<AppState>,
) -> Result<Json<PortfolioResponse>, (StatusCode, String)> {
    match repo.get_all_positions().await {
        Ok(positions) => Ok(Json(build_portfolio_response(&positions))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve portfolio: {err}"),
        )),
    }
}

/// Payload for updating real-time asset mark-to-market prices.
#[derive(Debug, Deserialize)]
pub struct UpdatePricePayload {
    pub price: f64,
}

/// HTTP Handler for `POST /api/positions/:ticker/price`.
///
/// Updates the real-time price of an asset, triggering instant mark-to-market recalculations.
pub async fn update_asset_price(
    State(repo): State<AppState>,
    Path(ticker): Path<String>,
    Json(payload): Json<UpdatePricePayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    match repo.update_price(&ticker, payload.price).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(err) => Err((StatusCode::NOT_FOUND, format!("Update failed: {err}"))),
    }
}
