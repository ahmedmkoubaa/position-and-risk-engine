use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use position_and_risk_engine::{
    create_app, InMemoryPositionRepository, PortfolioResponse, PositionRepository,
};
use std::sync::Arc;
use tower::ServiceExt;

const EPSILON: f64 = 1e-6;

#[tokio::test]
async fn test_get_portfolio_endpoint_matches_mock_data() {
    let repo = Arc::new(InMemoryPositionRepository::new());
    let app = create_app(repo);

    let request = Request::builder()
        .uri("/api/portfolio")
        .method("GET")
        .body(Body::empty())
        .expect("Failed to build HTTP request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to execute oneshot request");

    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .expect("Missing content-type header");
    assert!(content_type.contains("application/json"));

    let body_bytes = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();

    let portfolio_response: PortfolioResponse =
        serde_json::from_slice(&body_bytes).expect("Failed to deserialize JSON response");

    assert_eq!(portfolio_response.positions.len(), 11);
    assert_eq!(portfolio_response.summary.total_positions, 11);
    assert_eq!(portfolio_response.history.len(), 7);

    // Verify key assets from mock data
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
}

#[tokio::test]
async fn test_update_asset_price_updates_repo_state() {
    let repo = Arc::new(InMemoryPositionRepository::new());

    // Update BTC price from 55,000 to 65,000
    repo.update_price("BTC", 65000.0)
        .await
        .expect("Failed to update BTC price");

    let positions = repo.get_all_positions().await.unwrap();
    let btc = positions.iter().find(|p| p.ticker == "BTC").unwrap();
    assert_eq!(btc.current_price, 65000.0);
    // PnL should now be (65000 - 60000) * 0.5 = +2500.0
    assert_eq!(btc.calculate_pnl(), 2500.0);
}

#[tokio::test]
async fn test_root_serves_static_dashboard_html() {
    let repo = Arc::new(InMemoryPositionRepository::new());
    let app = create_app(repo);

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
        body_str.contains("Position & Risk Engine"),
        "HTML must contain the dashboard title"
    );
}
