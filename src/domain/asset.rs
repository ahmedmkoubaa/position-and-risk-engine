use serde::{Deserialize, Serialize};

/// Represents the classification of a financial asset in the portfolio.
///
/// FinTech systems categorize instruments to apply specialized risk models,
/// regulatory capital requirements, and margin calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    /// Equities / Stocks (e.g., AAPL, MSFT, TSLA)
    Share,
    /// Fixed income securities / Debt instruments (e.g., US10Y Treasury Bond)
    Bond,
    /// Digital assets and cryptocurrencies (e.g., BTC, ETH)
    Crypto,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetType::Share => write!(f, "Share"),
            AssetType::Bond => write!(f, "Bond"),
            AssetType::Crypto => write!(f, "Crypto"),
        }
    }
}
