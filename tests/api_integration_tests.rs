use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use position_and_risk_engine::{create_app, mock_data, PortfolioResponse};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceExt; // for oneshot

const EPSILON: f64 = 1e-6;

/// Integration Test for `GET /api/portfolio` endpoint.
///
/// Tests the full HTTP request lifecycle through Axum, Tokio state synchronization,
/// mock data loading from `mock_data.rs`, and JSON serialization.
#[tokio::test]
async fn test_get_portfolio_endpoint_matches_mock_data() {
    // 1. Arrange: Load mock data from mock_data.rs into shared thread-safe state
    let initial_positions = mock_data::get_mock_positions();
    let expected_count = initial_positions.len();
    assert_eq!(expected_count, 11, "Expected 11 mock positions in dataset");

    let state = Arc::new(RwLock::new(initial_positions));
    let app = create_app(state);

    // 2. Act: Dispatch HTTP GET /api/portfolio using in-memory oneshot channel
    let request = Request::builder()
        .uri("/api/portfolio")
        .method("GET")
        .body(Body::empty())
        .expect("Failed to build HTTP request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to execute oneshot request");

    // 3. Assert HTTP Status and Headers
    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .expect("Missing content-type header");
    assert!(
        content_type.contains("application/json"),
        "Expected application/json, got {content_type}"
    );

    // 4. Assert Payload Serialization and Mathematical Correctness
    let body_bytes = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();

    let portfolio_response: PortfolioResponse =
        serde_json::from_slice(&body_bytes).expect("Failed to deserialize JSON response");

    // Verify positions count
    assert_eq!(portfolio_response.positions.len(), 11);
    assert_eq!(portfolio_response.summary.total_positions, 11);

    // Verify key assets from mock_data.rs
    let aapl = portfolio_response
        .positions
        .iter()
        .find(|p| p.ticker == "AAPL")
        .expect("AAPL position not found");
    assert!((aapl.pnl - 200.0).abs() < EPSILON);
    assert!((aapl.exposure - 1700.0).abs() < EPSILON);

    let btc = portfolio_response
        .positions
        .iter()
        .find(|p| p.ticker == "BTC")
        .expect("BTC position not found");
    assert!((btc.pnl - (-2500.0)).abs() < EPSILON);
    assert!((btc.exposure - 27500.0).abs() < EPSILON);

    let us10y = portfolio_response
        .positions
        .iter()
        .find(|p| p.ticker == "US10Y")
        .expect("US10Y position not found");
    assert!((us10y.pnl - 250.0).abs() < EPSILON);
    assert!((us10y.exposure - 5250.0).abs() < EPSILON);

    // Verify total aggregate summary matches sum of items
    let sum_pnl: f64 = portfolio_response.positions.iter().map(|p| p.pnl).sum();
    let sum_exposure: f64 = portfolio_response.positions.iter().map(|p| p.exposure).sum();

    assert!(
        (portfolio_response.summary.total_pnl - sum_pnl).abs() < EPSILON,
        "Total PnL aggregate must match sum of individual PnLs"
    );
    assert!(
        (portfolio_response.summary.total_exposure - sum_exposure).abs() < EPSILON,
        "Total Exposure aggregate must match sum of individual exposures"
    );
}

/// Integration Test for serving the static HTML dashboard at `GET /`.
#[tokio::test]
async fn test_root_serves_static_dashboard_html() {
    let initial_positions = mock_data::get_mock_positions();
    let state = Arc::new(RwLock::new(initial_positions));
    let app = create_app(state);

    let request = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .expect("Failed to build HTTP request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to execute oneshot request");

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();

    let body_str = String::from_utf8_lossy(&body_bytes);
    assert!(
        body_str.contains("Risk & PnL Dashboard"),
        "HTML must contain the dashboard title"
    );
    assert!(
        body_str.contains("positions-table-body"),
        "HTML must contain the table body container"
    );
}
