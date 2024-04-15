use crate::domain::{DatastoreConnectError, Details};
use crate::libraries::sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;

#[non_exhaustive]
pub struct Connector {
    pub(super) client: SqlitePool,
}

impl Connector {
    #[must_use]
    pub fn get_client(&self) -> &SqlitePool {
        &self.client
    }

    pub(crate) async fn construct_client(
        details: &Details,
    ) -> Result<Connector, DatastoreConnectError> {
        Ok(Connector {
            client: SqlitePool::connect_with(
                SqliteConnectOptions::new()
                    .filename(if details.name.is_empty() {
                        ":memory:".to_string()
                    } else {
                        format!("{}.sqlite", details.name)
                    })
                    .create_if_missing(true)
                    .foreign_keys(true),
            )
            .await?,
        })
    }
}
