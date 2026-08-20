use super::asset::AssetType;
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
    /// Total exposure allocated in Equities / Shares
    pub shares_exposure: f64,
    /// Total exposure allocated in Crypto
    pub crypto_exposure: f64,
    /// Total exposure allocated in Fixed Income / Bonds
    pub bonds_exposure: f64,
}

/// Historical valuation point for chart timeseries visualization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryPoint {
    pub label: String,
    pub total_exposure: f64,
    pub total_pnl: f64,
}

/// Unified API response envelope delivering positions, summary, and historical chart trend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub summary: PortfolioSummary,
    pub positions: Vec<PositionView>,
    pub history: Vec<HistoryPoint>,
}

/// Aggregates a slice of positions into a complete `PortfolioResponse`.
pub fn build_portfolio_response(positions: &[Position]) -> PortfolioResponse {
    let mut total_pnl = 0.0;
    let mut total_exposure = 0.0;
    let mut total_cost_basis = 0.0;

    let mut shares_exposure = 0.0;
    let mut crypto_exposure = 0.0;
    let mut bonds_exposure = 0.0;

    let position_views: Vec<PositionView> = positions
        .iter()
        .map(|pos| {
            let view = pos.to_view();
            total_pnl += view.pnl;
            total_exposure += view.exposure;
            total_cost_basis += pos.buy_price * pos.quantity;

            match pos.asset_type {
                AssetType::Share => shares_exposure += view.exposure,
                AssetType::Crypto => crypto_exposure += view.exposure,
                AssetType::Bond => bonds_exposure += view.exposure,
            }

            view
        })
        .collect();

    let total_pnl_percentage = if total_cost_basis > 0.0 {
        (total_pnl / total_cost_basis) * 100.0
    } else {
        0.0
    };

    // Generate historical trend curve for interactive charting
    let history = generate_sample_history(total_exposure, total_pnl);

    PortfolioResponse {
        summary: PortfolioSummary {
            total_pnl,
            total_exposure,
            total_positions: positions.len(),
            total_pnl_percentage,
            shares_exposure,
            crypto_exposure,
            bonds_exposure,
        },
        positions: position_views,
        history,
    }
}

/// Generates a realistic 7-interval intraday historical timeseries curve leading to current valuation.
fn generate_sample_history(current_exposure: f64, current_pnl: f64) -> Vec<HistoryPoint> {
    let intervals = ["09:30", "11:00", "12:30", "14:00", "15:30", "16:45", "Current (Live)"];
    let multipliers = [0.978, 0.985, 0.992, 0.988, 0.995, 1.002, 1.0];
    let pnl_offsets = [-450.0, -320.0, -150.0, -280.0, -50.0, 80.0, 0.0];

    intervals
        .iter()
        .zip(multipliers.iter())
        .zip(pnl_offsets.iter())
        .map(|((&label, &m), &offset)| HistoryPoint {
            label: label.to_string(),
            total_exposure: (current_exposure * m).round(),
            total_pnl: current_pnl + offset,
        })
        .collect()
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

        assert_eq!(response.positions.len(), 3);
        assert!((response.summary.total_pnl - (-2050.0)).abs() < EPSILON);
        assert!((response.summary.total_exposure - 34450.0).abs() < EPSILON);
        assert!((response.summary.shares_exposure - 1700.0).abs() < EPSILON);
        assert!((response.summary.crypto_exposure - 27500.0).abs() < EPSILON);
        assert!((response.summary.bonds_exposure - 5250.0).abs() < EPSILON);
        assert_eq!(response.history.len(), 7);
    }
}
