use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::domain::{DatastoreConnectError, Details};

#[non_exhaustive]
pub struct Connector {
    pub(super) client: Surreal<Client>,
}

impl Connector {
    #[must_use]
    pub fn get_client(&self) -> &Surreal<Client> {
        &self.client
    }

    pub(crate) async fn construct_client(
        details: &Details,
    ) -> Result<Connector, DatastoreConnectError> {
        let connection_string = format!("{}:{}", details.host, details.port);
        let client = if details.secure {
            Surreal::new::<Wss>(connection_string)
        } else {
            Surreal::new::<Ws>(connection_string)
        }
        .await
        .map_err(|e| DatastoreConnectError::SurrealError(e.to_string()))?;

        client
            .signin(Root {
                username: &details.user,
                password: &details.password,
            })
            .await
            .map_err(|e| DatastoreConnectError::SurrealError(e.to_string()))?;

        client
            .use_ns(&details.name)
            .use_db(&details.name)
            .await
            .map_err(|e| DatastoreConnectError::SurrealError(e.to_string()))?;
        Ok(Connector { client })
    }
}
