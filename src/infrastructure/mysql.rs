use crate::domain::{DatastoreConnectError, Details};
use crate::libraries::sqlx::mysql::MySqlConnectOptions;
use crate::libraries::sqlx::mysql::MySqlSslMode::{Preferred, Required};
use crate::libraries::sqlx::MySqlPool;

#[non_exhaustive]
pub struct Connector {
    pub(super) client: MySqlPool,
}

impl Connector {
    #[must_use]
    pub fn get_client(&self) -> &MySqlPool {
        &self.client
    }

    pub(crate) async fn construct_client(
        details: &Details,
    ) -> Result<Connector, DatastoreConnectError> {
        Ok(Connector {
            client: MySqlPool::connect_with(
                MySqlConnectOptions::new()
                    .host(&details.host)
                    .port(details.port)
                    .database(&details.name)
                    .username(&details.user)
                    .password(&details.password)
                    .ssl_mode(if details.secure { Required } else { Preferred }),
            )
            .await?,
        })
    }
}
