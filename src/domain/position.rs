use super::asset::AssetType;
use serde::{Deserialize, Serialize};

/// Represents an individual financial position held within the portfolio.
///
/// Encapsulates the core trade parameters required to evaluate market valuation,
/// exposure risk, and unrealized profit & loss.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// Asset symbol / identifier (e.g., "AAPL", "BTC", "US10Y")
    pub ticker: String,
    /// Classification of the financial instrument
    pub asset_type: AssetType,
    /// Number of units held (supports fractional units for crypto or precision assets)
    pub quantity: f64,
    /// Weighted average acquisition price per unit
    pub buy_price: f64,
    /// Current real-time market price per unit
    pub current_price: f64,
}

/// Data Transfer Object (DTO) for presenting position data with computed metrics to the client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionView {
    pub ticker: String,
    pub asset_type: AssetType,
    pub quantity: f64,
    pub buy_price: f64,
    pub current_price: f64,
    /// Total capital at risk in current market terms: `current_price * quantity`
    pub exposure: f64,
    /// Unrealized Profit and Loss (PnL): `(current_price - buy_price) * quantity`
    pub pnl: f64,
    /// Percentage return on investment: `((current_price - buy_price) / buy_price) * 100.0`
    pub pnl_percentage: f64,
}

impl Position {
    /// Creates a new `Position` with the specified parameters.
    pub fn new(
        ticker: impl Into<String>,
        asset_type: AssetType,
        quantity: f64,
        buy_price: f64,
        current_price: f64,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            asset_type,
            quantity,
            buy_price,
            current_price,
        }
    }

    /// Calculates Unrealized Profit and Loss (PnL).
    ///
    /// # Formula
    /// $$\text{PnL} = (\text{Current Price} - \text{Buy Price}) \times \text{Quantity}$$
    ///
    /// Returns a positive value for profit (green) and negative for loss (red).
    #[inline]
    pub fn calculate_pnl(&self) -> f64 {
        (self.current_price - self.buy_price) * self.quantity
    }

    /// Calculates percentage return on investment (PnL %).
    ///
    /// Handles zero buy_price gracefully by returning 0.0 to prevent `NaN` / division by zero.
    #[inline]
    pub fn calculate_pnl_percentage(&self) -> f64 {
        if self.buy_price == 0.0 {
            0.0
        } else {
            ((self.current_price - self.buy_price) / self.buy_price) * 100.0
        }
    }

    /// Calculates total financial exposure (market value of the position).
    ///
    /// # Formula
    /// $$\text{Exposure} = \text{Current Price} \times \text{Quantity}$$
    ///
    /// Represents the gross capital allocated to this asset at current market price.
    #[inline]
    pub fn calculate_exposure(&self) -> f64 {
        self.current_price * self.quantity
    }

    /// Transforms the domain model into a computed `PositionView` ready for API serialization.
    pub fn to_view(&self) -> PositionView {
        PositionView {
            ticker: self.ticker.clone(),
            asset_type: self.asset_type,
            quantity: self.quantity,
            buy_price: self.buy_price,
            current_price: self.current_price,
            exposure: self.calculate_exposure(),
            pnl: self.calculate_pnl(),
            pnl_percentage: self.calculate_pnl_percentage(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_equity_profit_calculation_aapl() {
        // AAPL: Buy 10 units @ $150.0, current @ $170.0
        // Expected PnL: (170 - 150) * 10 = +200.0
        // Expected Exposure: 170 * 10 = 1700.0
        // Expected PnL %: ((170 - 150) / 150) * 100 = +13.333333%
        let position = Position::new("AAPL", AssetType::Share, 10.0, 150.0, 170.0);

        let pnl = position.calculate_pnl();
        let exposure = position.calculate_exposure();
        let pnl_pct = position.calculate_pnl_percentage();

        assert!((pnl - 200.0).abs() < EPSILON, "Expected PnL of 200.0, got {pnl}");
        assert!((exposure - 1700.0).abs() < EPSILON, "Expected Exposure of 1700.0, got {exposure}");
        assert!((pnl_pct - (20.0 / 150.0 * 100.0)).abs() < EPSILON, "Expected correct PnL %");
    }

    #[test]
    fn test_crypto_loss_calculation_btc() {
        // BTC: Buy 0.5 units @ $60,000.0, current @ $55,000.0
        // Expected PnL: (55000 - 60000) * 0.5 = -2500.0
        // Expected Exposure: 55000 * 0.5 = 27500.0
        // Expected PnL %: ((55000 - 60000) / 60000) * 100 = -8.333333%
        let position = Position::new("BTC", AssetType::Crypto, 0.5, 60000.0, 55000.0);

        let pnl = position.calculate_pnl();
        let exposure = position.calculate_exposure();
        let pnl_pct = position.calculate_pnl_percentage();

        assert!((pnl - (-2500.0)).abs() < EPSILON, "Expected PnL of -2500.0, got {pnl}");
        assert!((exposure - 27500.0).abs() < EPSILON, "Expected Exposure of 27500.0, got {exposure}");
        assert!((pnl_pct - (-5000.0 / 60000.0 * 100.0)).abs() < EPSILON, "Expected correct PnL %");
    }

    #[test]
    fn test_bond_gain_calculation_us10y() {
        // US10Y: Buy 50 units @ $100.0, current @ $105.0
        // Expected PnL: (105 - 100) * 50 = +250.0
        // Expected Exposure: 105 * 50 = 5250.0
        let position = Position::new("US10Y", AssetType::Bond, 50.0, 100.0, 105.0);

        let pnl = position.calculate_pnl();
        let exposure = position.calculate_exposure();

        assert!((pnl - 250.0).abs() < EPSILON, "Expected PnL of 250.0, got {pnl}");
        assert!((exposure - 5250.0).abs() < EPSILON, "Expected Exposure of 5250.0, got {exposure}");
    }

    #[test]
    fn test_zero_quantity_edge_case() {
        let position = Position::new("CASH", AssetType::Share, 0.0, 100.0, 120.0);
        assert_eq!(position.calculate_pnl(), 0.0);
        assert_eq!(position.calculate_exposure(), 0.0);
    }

    #[test]
    fn test_zero_buy_price_edge_case() {
        let position = Position::new("AIRDROP", AssetType::Crypto, 10.0, 0.0, 50.0);
        assert_eq!(position.calculate_pnl(), 500.0);
        assert_eq!(position.calculate_pnl_percentage(), 0.0); // Guards against div by zero
        assert_eq!(position.calculate_exposure(), 500.0);
    }
}
