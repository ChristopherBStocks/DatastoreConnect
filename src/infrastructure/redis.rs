use crate::domain::{DatastoreConnectError, Details};
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;

#[non_exhaustive]
pub struct Connector {
    pub(super) client: Pool<RedisConnectionManager>,
}

impl Connector {
    #[must_use]
    pub fn get_client(&self) -> Pool<RedisConnectionManager> {
        self.client.clone()
    }

    pub(crate) async fn construct_client(
        details: &Details,
    ) -> Result<Connector, DatastoreConnectError> {
        let connection_string = format!(
            "{}://{}{}@{}{}",
            if details.secure { "rediss" } else { "redis" },
            details.user,
            if details.password.is_empty() {
                "".into()
            } else {
                format!(":{}", details.password)
            },
            details.host,
            details.port
        );
        let manager = RedisConnectionManager::new(connection_string)?;
        Ok(Connector {
            client: Pool::builder().build(manager).await?,
        })
    }
}
