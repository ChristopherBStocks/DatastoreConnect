use crate::domain::{DatastoreConnectError, Details};
use crate::libraries::sqlx::postgres::PgConnectOptions;
use crate::libraries::sqlx::postgres::PgSslMode::{Prefer, Require};
use crate::libraries::sqlx::PgPool;

#[non_exhaustive]
pub struct Connector {
    pub(super) client: PgPool,
}

impl Connector {
    #[must_use]
    pub fn get_client(&self) -> &PgPool {
        &self.client
    }

    pub(crate) async fn construct_client(
        details: &Details,
    ) -> Result<Connector, DatastoreConnectError> {
        Ok(Connector {
            client: PgPool::connect_with(
                PgConnectOptions::new()
                    .host(&details.host)
                    .port(details.port)
                    .database(&details.name)
                    .username(&details.user)
                    .password(&details.password)
                    .ssl_mode(if details.secure { Require } else { Prefer }),
            )
            .await?,
        })
    }
}
