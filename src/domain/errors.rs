use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatastoreConnectError {
    #[cfg(any(feature = "mysql", feature = "sqlite", feature = "postgres"))]
    #[error("DatastoreConnect // Sqlx // {0}")]
    SqlxError(#[from] sqlx::Error),

    #[cfg(feature = "redis")]
    #[error("DatastoreConnect // Redis // {0}")]
    RedisError(#[from] bb8_redis::redis::RedisError),

    #[cfg(feature = "surreal")]
    #[error("DatastoreConnect // Surreal // {0}")]
    SurrealError(String),

    #[error("DatastoreConnect // Invalid Extraction")]
    InvalidExtraction,
}
