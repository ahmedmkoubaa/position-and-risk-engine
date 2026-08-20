use crate::domain::{AssetType, Position};

/// Generates a realistic, diversified multi-asset portfolio dataset (11 positions)
/// spanning Global Equities, Sovereignty Bonds, and Cryptocurrencies.
///
/// Designed to showcase institutional-grade portfolio risk monitoring,
/// positive/negative PnL spread, and diverse asset allocation.
pub fn get_mock_positions() -> Vec<Position> {
    vec![
        // --- Equities / Shares ---
        Position::new("AAPL", AssetType::Share, 10.0, 150.0, 170.0),
        Position::new("NVDA", AssetType::Share, 15.0, 110.0, 135.0),
        Position::new("MSFT", AssetType::Share, 8.0, 420.0, 445.0),
        Position::new("TSLA", AssetType::Share, 12.0, 220.0, 195.0),
        Position::new("AMZN", AssetType::Share, 20.0, 180.0, 185.0),

        // --- Cryptocurrencies ---
        Position::new("BTC", AssetType::Crypto, 0.5, 60000.0, 55000.0),
        Position::new("ETH", AssetType::Crypto, 4.0, 3200.0, 3450.0),
        Position::new("SOL", AssetType::Crypto, 25.0, 140.0, 160.0),

        // --- Fixed Income & Sovereign Bonds ---
        Position::new("US10Y", AssetType::Bond, 50.0, 100.0, 105.0),
        Position::new("BUND10Y", AssetType::Bond, 40.0, 98.0, 96.5),
        Position::new("UKGILT", AssetType::Bond, 30.0, 102.0, 104.0),
    ]
}
