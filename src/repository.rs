use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::Position;
use crate::mock_data;

/// Custom error type for repository operations.
///
/// In production, this can map to SQL errors, connection pool timeouts,
/// or network failures with zero panic risk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    NotFound(String),
    Internal(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(ticker) => write!(f, "Position not found: {ticker}"),
            RepositoryError::Internal(msg) => write!(f, "Repository internal error: {msg}"),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Abstract Position Repository Trait (Equivalent to Java Repository Interface / Port in Hexagonal Architecture).
///
/// Decouples business logic from persistence technologies (Postgres, Redis, DynamoDB, Memory).
#[async_trait]
pub trait PositionRepository: Send + Sync {
    /// Retrieves all active positions from the underlying data store.
    async fn get_all_positions(&self) -> Result<Vec<Position>, RepositoryError>;

    /// Updates the mark-to-market current price of a position.
    async fn update_price(&self, ticker: &str, new_price: f64) -> Result<(), RepositoryError>;
}

/// In-Memory implementation of `PositionRepository` with thread-safe read/write locking.
///
/// Ready to be swapped with `PostgresPositionRepository` or `DynamoDbPositionRepository`
/// without modifying handlers or domain logic.
#[derive(Clone)]
pub struct InMemoryPositionRepository {
    positions: Arc<RwLock<Vec<Position>>>,
}

impl InMemoryPositionRepository {
    /// Creates a repository initialized with default mock portfolio holdings.
    pub fn new() -> Self {
        Self {
            positions: Arc::new(RwLock::new(mock_data::get_mock_positions())),
        }
    }

    /// Creates a repository with custom initial positions (useful for integration testing).
    pub fn with_positions(positions: Vec<Position>) -> Self {
        Self {
            positions: Arc::new(RwLock::new(positions)),
        }
    }
}

impl Default for InMemoryPositionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PositionRepository for InMemoryPositionRepository {
    async fn get_all_positions(&self) -> Result<Vec<Position>, RepositoryError> {
        let lock = self.positions.read().await;
        Ok(lock.clone())
    }

    async fn update_price(&self, ticker: &str, new_price: f64) -> Result<(), RepositoryError> {
        let mut lock = self.positions.write().await;
        if let Some(pos) = lock.iter_mut().find(|p| p.ticker.eq_ignore_ascii_case(ticker)) {
            pos.current_price = new_price;
            Ok(())
        } else {
            Err(RepositoryError::NotFound(ticker.to_string()))
        }
    }
}
