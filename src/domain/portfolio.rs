use super::position::{Position, PositionView};
use serde::{Deserialize, Serialize};

/// High-level portfolio risk and performance metrics summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioSummary {
    /// Aggregate Unrealized Profit & Loss across all active positions
    pub total_pnl: f64,
    /// Gross total market exposure across all active positions
    pub total_exposure: f64,
    /// Total number of distinct asset positions held
    pub total_positions: usize,
    /// Overall portfolio return on capital percentage
    pub total_pnl_percentage: f64,
}

/// Unified API response envelope delivering both individual positions and aggregate summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub summary: PortfolioSummary,
    pub positions: Vec<PositionView>,
}

/// Aggregates a slice of positions into a complete `PortfolioResponse`.
///
/// Ensures safe arithmetic without `.unwrap()`, calculating both individual
/// position views and the portfolio-wide executive summary.
pub fn build_portfolio_response(positions: &[Position]) -> PortfolioResponse {
    let mut total_pnl = 0.0;
    let mut total_exposure = 0.0;
    let mut total_cost_basis = 0.0;

    let position_views: Vec<PositionView> = positions
        .iter()
        .map(|pos| {
            let view = pos.to_view();
            total_pnl += view.pnl;
            total_exposure += view.exposure;
            total_cost_basis += pos.buy_price * pos.quantity;
            view
        })
        .collect();

    let total_pnl_percentage = if total_cost_basis > 0.0 {
        (total_pnl / total_cost_basis) * 100.0
    } else {
        0.0
    };

    PortfolioResponse {
        summary: PortfolioSummary {
            total_pnl,
            total_exposure,
            total_positions: positions.len(),
            total_pnl_percentage,
        },
        positions: position_views,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::asset::AssetType;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_aggregate_portfolio_summary() {
        let positions = vec![
            Position::new("AAPL", AssetType::Share, 10.0, 150.0, 170.0), // PnL: +200, Exp: 1700
            Position::new("BTC", AssetType::Crypto, 0.5, 60000.0, 55000.0), // PnL: -2500, Exp: 27500
            Position::new("US10Y", AssetType::Bond, 50.0, 100.0, 105.0), // PnL: +250, Exp: 5250
        ];

        let response = build_portfolio_response(&positions);

        // Expected Total PnL: 200 - 2500 + 250 = -2050.0
        // Expected Total Exposure: 1700 + 27500 + 5250 = 34450.0
        // Cost basis: (150*10) + (60000*0.5) + (100*50) = 1500 + 30000 + 5000 = 36500.0
        // PnL %: (-2050 / 36500) * 100 = -5.6164%
        assert_eq!(response.positions.len(), 3);
        assert!((response.summary.total_pnl - (-2050.0)).abs() < EPSILON);
        assert!((response.summary.total_exposure - 34450.0).abs() < EPSILON);
        assert_eq!(response.summary.total_positions, 3);
    }
}
