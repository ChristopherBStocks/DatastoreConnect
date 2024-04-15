use crate::domain::{Datastore, DatastoreConnectError};
use crate::infrastructure;

impl Datastore {
    #[cfg(feature = "mysql")]
    pub fn mysql(&self) -> Result<&infrastructure::MySQLConnector, DatastoreConnectError> {
        match self {
            Datastore::MySQL(connector) => Ok(connector),
            _ => Err(DatastoreConnectError::InvalidExtraction),
        }
    }

    #[cfg(feature = "postgres")]
    pub fn postgres(&self) -> Result<&infrastructure::PostgresConnector, DatastoreConnectError> {
        match self {
            Datastore::Postgres(connector) => Ok(connector),
            _ => Err(DatastoreConnectError::InvalidExtraction),
        }
    }

    #[cfg(feature = "redis")]
    pub fn redis(&self) -> Result<&infrastructure::RedisConnector, DatastoreConnectError> {
        match self {
            Datastore::Redis(connector) => Ok(connector),
            _ => Err(DatastoreConnectError::InvalidExtraction),
        }
    }

    #[cfg(feature = "sqlite")]
    pub fn sqlite(&self) -> Result<&infrastructure::SQLiteConnector, DatastoreConnectError> {
        match self {
            Datastore::SQLite(connector) => Ok(connector),
            _ => Err(DatastoreConnectError::InvalidExtraction),
        }
    }

    #[cfg(feature = "surreal")]
    pub fn surreal(&self) -> Result<&infrastructure::SurrealConnector, DatastoreConnectError> {
        match self {
            Datastore::Surreal(connector) => Ok(connector),
            _ => Err(DatastoreConnectError::InvalidExtraction),
        }
    }
}