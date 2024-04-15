use crate::domain::{Datastore, DatastoreConnectError, Details};
use env_utils::libraries::dotenvy::var;
use crate::builders::DetailsBuilder;
use crate::infrastructure;

impl Details {
    pub async fn from_env(
        prefix: &str
    ) -> Result<Datastore, DatastoreConnectError> {
        DetailsBuilder::from_env(prefix).build().datastore(prefix).await
    }

    pub async fn datastore(
        &self,
        prefix: &str,
    ) -> Result<Datastore, DatastoreConnectError> {
        match var(format!("{}_TYPE", prefix).to_ascii_uppercase().as_str())
            .unwrap_or_else(|_| "disabled".to_string())
            .to_ascii_lowercase()
            .as_str()
        {
            #[cfg(feature = "sqlite")]
            "sqlite" => infrastructure::SQLiteConnector::construct_client(&self)
                .await
                .map(Datastore::SQLite),
            #[cfg(feature = "mysql")]
            "mysql" => infrastructure::MySQLConnector::construct_client(&self)
                .await
                .map(Datastore::MySQL),
            #[cfg(feature = "postgres")]
            "postgres" => infrastructure::PostgresConnector::construct_client(&self)
                .await
                .map(Datastore::Postgres),
            #[cfg(feature = "redis")]
            "redis" => infrastructure::RedisConnector::construct_client(&self)
                .await
                .map(Datastore::Redis),
            #[cfg(feature = "surreal")]
            "surreal" => infrastructure::SurrealConnector::construct_client(&self)
                .await
                .map(Datastore::Surreal),
            _ => Ok(Datastore::Disabled),
        }
    }
}
